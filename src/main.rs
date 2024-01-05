use chrono::{Datelike, NaiveDateTime};
use heatingpump::Metric;
use plotly::{BoxPlot, Plot};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn clamp(val: usize, min: usize, max: usize) -> usize {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

fn percentile(vec: &Vec<f64>, p: f64) -> f64 {
    let pos = vec.len() as f64 * p + 0.5;
    let idx = clamp(pos as usize, 0, vec.len());
    return vec[idx];
}

fn main() {
    let mut plot = Plot::new();
    let trace = BoxPlot::new(vec![0, 1, 2, 3, 4, 5])
        .box_points(plotly::box_plot::BoxPoints::All)
        .jitter(0.3)
        .point_pos(0.0);
    plot.add_trace(trace);

    plot.write_html("out.html");

    let now = Instant::now();
    let filename: &str = "/home/martinus/Documents/KEBA KeEnergy CSV files/2023-05-31T09:54:50 - 2024-01-03T13:12:27.csv";
    let mut counter: u32 = 0;

    let mut current_month_id: u32 = 0;
    let mut temp_outdoor: Metric = Metric::new();
    let mut room_set_temp_0 = Metric::new();
    let mut room_set_temp_1 = Metric::new();
    let mut room_temp_0 = Metric::new();
    let mut room_temp_1 = Metric::new();
    let mut electric_power = Metric::new();
    let mut heat_power = Metric::new();

    let mut date_time_prev: Option<NaiveDateTime> = None;

    let mut outdoor_temperatures: Vec<f64> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            let parts = line.split(';');
            let p = parts.collect::<Vec<_>>();

            // 17.6.2023 10:5:56
            // %d.%m.%Y %H:%M:%S
            let date_time_parse_result = NaiveDateTime::parse_from_str(p[0], "%d.%m.%Y %H:%M:%S");
            let date_time = match date_time_parse_result {
                Ok(data) => data,
                Err(error) => {
                    // first line!
                    eprintln!("Error parsing date '{}': {:?}", p[0], error);
                    continue;
                }
            };

            // now we got date_time!
            let month_id: u32 = (date_time.year() as u32) * 100 + date_time.month();

            if month_id != current_month_id {
                if temp_outdoor.count() > 0 {
                    let watt_seconds_to_kwh = 1.0 / (1000.0 * 60.0 * 60.0);
                    let electricity_kwh = electric_power.sum() * watt_seconds_to_kwh;
                    let heat_kw = heat_power.sum() * watt_seconds_to_kwh;
                    let cop = heat_power.sum() / electric_power.sum();

                    outdoor_temperatures.sort_by(|a, b| a.partial_cmp(b).unwrap());

                    println!(
                        "{:4}-{:02}: Außentemperatur: {:4.1}° Durchschnitt, {:4.1}° bis {:4.1}° | Oberhaus: {:4.1}° Soll, {:4.1}° Ist | Unterhaus {:4.1}° Soll, {:4.1}° Ist | Energie: {:5.1} kWh Strom, {:6.1} kWh Wärme => COP = {:4.2} | percentiles: {:5.1} {:5.1} {:5.1} {:5.1} {:5.1}",
                        current_month_id / 100,
                        current_month_id % 100,
                        temp_outdoor.avg(),
                        temp_outdoor.min(),
                        temp_outdoor.max(),
                        room_set_temp_1.avg(),
                        room_temp_1.avg(),
                        room_set_temp_0.avg(),
                        room_temp_0.avg(),
                        electricity_kwh,
                        heat_kw,
                        cop,
                        percentile(&outdoor_temperatures, 0.05),
                        percentile(&outdoor_temperatures, 0.25),
                        percentile(&outdoor_temperatures, 0.50),
                        percentile(&outdoor_temperatures, 0.75),
                        percentile(&outdoor_temperatures, 0.95)
                    );
                }

                current_month_id = month_id;
                temp_outdoor = Metric::new();
                room_set_temp_0 = Metric::new();
                room_set_temp_1 = Metric::new();
                room_temp_0 = Metric::new();
                room_temp_1 = Metric::new();
                electric_power = Metric::new();
                heat_power = Metric::new();
                outdoor_temperatures.clear();
            }

            let prev = match date_time_prev {
                Some(prev) => prev,
                None => {
                    date_time_prev = Some(date_time);
                    continue;
                }
            };

            let duration_s: u64 = (date_time - prev).num_seconds() as u64;
            let t = f64::from_str(p[50]).unwrap();
            outdoor_temperatures.resize(outdoor_temperatures.len() + duration_s as usize, t);

            date_time_prev = Some(date_time);

            temp_outdoor.add_times(p[50], duration_s);
            room_set_temp_0.add_times(p[60], duration_s);
            room_set_temp_1.add_times(p[192], duration_s);
            room_temp_0.add_times(p[92], duration_s);
            room_temp_1.add_times(p[147], duration_s);
            electric_power.add_times(p[122], duration_s);
            heat_power.add_times(p[76], duration_s);

            // heatpump[0].HeatPower 76
            // heatCircuit[0].RoomTemp 92
            // heatCircuit[1].RoomTemp 147
            // heatCircuit[1].RoomOffsetTemp 120
            // heatCircuit[1].RoomSetTemp 192 // actual temperature set (with offset combined)
            // heatpump[0].ElectricPower 122
            // heatpump[0].PwrCtrl.CalcPower 168
            // println!("{}: power={}", date_time, p[122]);

            counter += 1;
        }
    }
    println!("num lines={}, took {:.2?}", counter, now.elapsed());
}
