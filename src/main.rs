use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, NaiveTime};
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

fn main() {
    let now = Instant::now();
    let filename: &str = "/home/martinus/Documents/KEBA KeEnergy CSV files/2023-05-31T09:54:50 - 2024-01-03T13:12:27.csv";
    let mut counter: u32 = 0;

    let mut current_month_id: u32 = 0;
    let mut current_temp_sum: f64 = 0.0;
    let mut current_temp_count: u32 = 0;
    let mut current_temp_min: f64 = 999.0;
    let mut current_temp_max: f64 = -999.0;

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
            let temp_outdoor = f64::from_str(p[50]).unwrap();
            let month_id: u32 = (date_time.year() as u32) * 100 + date_time.month();

            if month_id != current_month_id {
                println!(
                    "{}: {}, min={}, max={}",
                    current_month_id,
                    current_temp_sum / current_temp_count as f64,
                    current_temp_min,
                    current_temp_max
                );

                current_month_id = month_id;
                current_temp_sum = 0.0;
                current_temp_count = 0;
                current_temp_min = 999.0;
                current_temp_max = -999.0;
            }

            current_temp_sum += temp_outdoor;
            current_temp_count += 1;
            if temp_outdoor > current_temp_max {
                current_temp_max = temp_outdoor;
            }
            if temp_outdoor < current_temp_min {
                current_temp_min = temp_outdoor;
            }

            // heatpump[0].HeatPower 76
            // heatCircuit[0].RoomTemp 92
            // heatCircuit[1].RoomTemp 147
            // heatCircuit[1].RoomOffsetTemp 120
            // heatCircuit[1].RoomSetTemp 192 // actual temperature set (with offset combined)
            // heatpump[0].ElectricPower 122
            // heatpump[0].PwrCtrl.CalcPower 168
            // println!("{}: RoomTemp={}, RoomSetTemp={}, RoomOffsetTemp={}", date_time, p[147], p[192], p[120]);

            /*
            for (i, part) in parts.enumerate() {
                if i == 50 {
                    println!("i={}, part=\"{}\"", i, part);
                }
            }
            */
            counter += 1;
        }
    }
    println!("num lines={}, took {:.2?}", counter, now.elapsed());
}
