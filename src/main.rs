use chrono::{Datelike, NaiveDateTime};
use heatingpump::Metric;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let now = Instant::now();
    let filename: &str = "/home/martinus/Documents/KEBA KeEnergy CSV files/2023-05-31T09:54:50 - 2024-01-03T13:12:27.csv";
    let mut counter: u32 = 0;

    let mut current_month_id: u32 = 0;
    let mut temp_outdoor: Metric = Metric::new();
    let mut room_set_temp_0 = Metric::new();
    let mut room_set_temp_1 = Metric::new();
    let mut room_temp_0 = Metric::new();
    let mut room_temp_1 = Metric::new();

    let mut heat_power = Metric::new();

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
                    println!(
                        "{}: Außentemperatur: {:4.1}° Durchschnitt, {:4.1}° bis {:4.1}° | Oberhaus: {:4.1}° Soll, {:4.1}° Ist | Unterhaus {:4.1}° Soll, {:4.1}° Ist | HeatPower={}",
                        current_month_id,
                        temp_outdoor.avg(),
                        temp_outdoor.min(),
                        temp_outdoor.max(),
                        room_set_temp_1.avg(),
                        room_temp_1.avg(),
                        room_set_temp_0.avg(),
                        room_temp_0.avg(),
                        heat_power.avg()
                    );
                }

                current_month_id = month_id;
                temp_outdoor = Metric::new();
                room_set_temp_0 = Metric::new();
                room_set_temp_1 = Metric::new();
                room_temp_0 = Metric::new();
                room_temp_1 = Metric::new();
                heat_power = Metric::new();
            }

            temp_outdoor.add_str(p[50]);
            room_set_temp_0.add_str(p[60]);
            room_set_temp_1.add_str(p[192]);
            room_temp_0.add_str(p[92]);
            room_temp_1.add_str(p[147]);
            heat_power.add_str(p[122]);

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
