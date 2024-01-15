use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/nicolas/Documents/Dev/Cpp/zmap/test2/output_2.csv")?;
    let mut write_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("new_output.csv")?;
    let reader = BufReader::new(file);

    let mut c: u64 = 0;
    let mut first = true;
    let mut max: u64 = u64::MIN;
    let mut min: u64 = u64::MAX;
    for line in reader.lines() {
        c += 1;
        if c % 100000 == 0 {
            println!("{}", c);
        }
        if first {
            first = false;
            continue;
        }
        let line = line.unwrap();
        let mut q = line.split(",");
        let ip = q.next().unwrap();
        let num = q.next().unwrap().parse::<u64>().unwrap();
        if num < min {
            min = num.clone();
        }
        if num > max {
            max = num;
        }
        write_file
            .write_all(format!("{} {}\n", ip, num).as_bytes())
            .unwrap();
    }

    println!("Min: {} | Max: {}", min, max);
    Ok(())
}
