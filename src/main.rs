use std::error::Error;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>>
{
    //let mut collected_voltage: Vec<String> = Vec::new();
    //let mut collected_current: Vec<String> = Vec::new();

    // TODO: automatically detect battery folders
    let mut file_voltage = File::open("/sys/class/power_supply/BAT1/voltage_now")?;
    let mut file_current = File::open("/sys/class/power_supply/BAT1/current_now")?;

    let mut voltage_str: String;
    let mut current_str: String;

    //collected_voltage.reserve(600);    // may be useful later
    //collected_current.reserve(600);
    
    let dataset_size = 600;    // TODO: set dataset and thread sleep size via CLI parameters

    for count in 0..dataset_size
    {
        file_voltage.seek(SeekFrom::Start(0))?;
        file_current.seek(SeekFrom::Start(0))?;
        voltage_str = String::new();
        current_str = String::new();
        file_voltage.read_to_string(&mut voltage_str)?;
        file_current.read_to_string(&mut current_str)?;
        let voltage_f64 = voltage_str.trim().parse::<f64>()? / 1_000_000.0;
        let current_f64 = current_str.trim().parse::<f64>()? / 1_000_000.0;
        //collected_current.push(current);    // may be useful later
        //collected_voltage.push(voltage);
        let wattage_f64 = voltage_f64 * current_f64;
        println!("{}",&wattage_f64.to_string());
        thread::sleep(time::Duration::from_secs(3));
    }

    // This code piece may be useful later for zero overhead from outputting data
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
