use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>>
{
    let mut collected_voltage: Vec<String> = Vec::new();
    let mut collected_current: Vec<String> = Vec::new();

    let mut file_voltage = File::open("/sys/class/power_supply/BAT1/voltage_now")?;
    let mut file_current = File::open("/sys/class/power_supply/BAT1/current_now")?;

    let mut voltage = String::new();
    let mut current = String::new();

    collected_voltage.reserve(600);
    collected_current.reserve(600);
    let dataset_size = 600;

    for count in 0..dataset_size
    {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;
        voltage = String::new();
        current = String::new();
        file_voltage.read_to_string(&mut voltage)?;
        file_current.read_to_string(&mut current)?;
        let voltage_f64 = voltage.trim().parse::<f64>()? / 1_000_000.0;
        let current_f64 = current.trim().parse::<f64>()? / 1_000_000.0;
        //collected_current.push(current);
        //collected_voltage.push(voltage);
        let wattage = voltage_f64 * current_f64;
        println!("{}",&wattage.to_string());
        thread::sleep(time::Duration::from_secs(3));
    }

    //println!("data collected!");
/* 
    for count in 0..dataset_size
    {
        let voltage_f64 = collected_voltage[count].trim().parse::<f64>()? / 1_000_000.0;
        let current_f64 = collected_current[count].trim().parse::<f64>()? / 1_000_000.0;

        let wattage = voltage_f64 * current_f64;

        println!("{}",&wattage.to_string());

        //println!("teststestetset");
        
    }*/

    return Ok(())
}
