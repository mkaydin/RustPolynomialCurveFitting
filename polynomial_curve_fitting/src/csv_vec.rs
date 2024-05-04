extern crate csv;
use csv::Writer;
use std::fs::{create_dir, File};

pub fn csv_writer_controller(x: Vec<i32>, y: Vec<i32>) {
    let operation: Result<(), String> = csv_vec_writer(x,y);
    match operation {
        Ok(()) => println!("csv file written successfully"),
        Err(e) => println!("Error: {}", e),
    }
}

fn csv_vec_writer_with_headers(x: Vec<i32>, y: Vec<i32>) -> Result<(), String> {
    let file = File::create("polynomial.csv").map_err(|e| format!("Could not create csv file: {}", e))?;
    let mut writer = Writer::from_writer(file);

    // Write headers
    writer.write_record(&["Index", "X", "Y"]).map_err(|e| format!("Error writing record: {}", e))?;

    // Iterate over indices of x (assuming x and y have the same length)
    for (i, (&x_val, &y_val)) in x.iter().zip(y.iter()).enumerate() {
        // Write each pair of values as a separate row, starting index from 1
        writer.write_record(&[(i + 1).to_string(), x_val.to_string(), y_val.to_string()])
            .map_err(|e| format!("Error writing record: {}", e))?;
    }

    Ok(())
}

fn csv_vec_writer(x: Vec<i32>, y: Vec<i32>) -> Result<(), String> {
    let file = File::create("polynomial.csv").map_err(|e| format!("Could not create csv file: {}", e))?;
    let mut writer = Writer::from_writer(file);

    // Iterate over indices of x (assuming x and y have the same length)
    for (i, (&x_val, &y_val)) in x.iter().zip(y.iter()).enumerate() {
        // Write each pair of values as a separate row, starting index from 1
        writer.write_record(&[(i + 1).to_string(), x_val.to_string(), y_val.to_string()])
            .map_err(|e| format!("Error writing record: {}", e))?;
    }

    Ok(())
}