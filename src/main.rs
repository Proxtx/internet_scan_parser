use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("/home/nicolas/Documents/Dev/Cpp/zmap/test2/output_2.csv")?;
    let reader = BufReader::new(file);

    let mut first = true;
    let mut max: u64 = u64::MIN;
    let mut min: u64 = u64::MAX;
    for line in reader.lines() {
        if first {
            first = false;
            continue;
        }
        let line = line.unwrap();
        let mut q = line.split(",");
        q.next();
        let num = q.next().unwrap().parse::<u64>().unwrap();
        if num < min {
            min = num.clone();
        }
        if num > max {
            max = num;
        }
    }

    println!("Min: {} | Max: {}", min, max);
    Ok(())
}
