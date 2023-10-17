// Import required libraries and modules
use std::collections::HashMap;
use serde_json;
use lazy_static::lazy_static;
use std::cmp;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

// Define the global constant for minimum digits after the decimal point
lazy_static! {
    static ref MIN_DIGITS: usize = 6;
}

// Function to round a floating-point number to a specified number of decimal places
fn round_to(value: f64, decimals: usize) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

// Function to update OHLC data for a given symbol
pub fn update_ohlc(
    line: &str,
    symbol_data: &mut HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)>,
    time_window: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the JSON object from the input line
    let json: serde_json::Value = serde_json::from_str(line)?;

    // Extract the symbol and timestamp from the JSON object
    let symbol = json.get("s").and_then(|s| s.as_str()).ok_or("Symbol not found")?.to_string();
    let timestamp = json.get("T").and_then(|t| t.as_u64()).ok_or("Timestamp not found")?;

    // Get the previous OHLC data for the symbol from the HashMap
    let (prev_timestamp, prev_open, prev_high, prev_low, _prev_close) =
        symbol_data.get(&symbol).cloned().unwrap_or((0, None, None, None, None));

    // Calculate the time difference between the current and previous timestamps
    let time_diff = timestamp - prev_timestamp;

    // Parse the bid and ask prices from the JSON object and calculate the current price
    let b = json.get("b").and_then(|b| b.as_str()).ok_or("Bid price not found")?.parse::<f64>()?;
    let a = json.get("a").and_then(|a| a.as_str()).ok_or("Ask price not found")?.parse::<f64>()?;
    let b_int = (b * 10f64.powi(8)) as i64;
    let a_int = (a * 10f64.powi(8)) as i64;
    let current_price_int = (b_int + a_int) / 2;
    let current_price_temp = current_price_int as f64 / 10f64.powi(8);
    let current_price = round_to(current_price_temp, *MIN_DIGITS);

    // Update OHLC data based on time window and previous data
    if time_diff > time_window || prev_timestamp == 0 {
        let price = Some(current_price);

        symbol_data.insert(symbol.clone(), (timestamp, price, price, price, price));
    } else {
        let open = prev_open;
        let high = match (prev_high, current_price.partial_cmp(&prev_high.unwrap_or(current_price))) {
            (Some(prev_high), Some(cmp::Ordering::Less)) => Some(prev_high),
            _ => Some(current_price),
        };
        let low = match (prev_low, current_price.partial_cmp(&prev_low.unwrap_or(current_price))) {
            (Some(prev_low), Some(cmp::Ordering::Greater)) => Some(prev_low),
            _ => Some(current_price),
        };
        let close = Some(current_price);

        symbol_data.insert(symbol.clone(), (timestamp, open, high, low, close));
    }

    Ok(())
}

// Function to compare OHLC values with a given precision
fn compare_ohlc_values(
    output_json: &serde_json::Value,
    expected_json: &serde_json::Value,
    precision: usize,
) -> bool {
    // Extract OHLC values from the output JSON object and convert them to f64
    if let (
        Some(output_open),
        Some(output_high),
        Some(output_low),
        Some(output_close),
    ) = (
        output_json.get("open").and_then(|o| o.as_str()).and_then(|o| o.parse::<f64>().ok()),
        output_json.get("high").and_then(|h| h.as_str()).and_then(|h| h.parse::<f64>().ok()),
        output_json.get("low").and_then(|l| l.as_str()).and_then(|l| l.parse::<f64>().ok()),
        output_json.get("close").and_then(|c| c.as_str()).and_then(|c| c.parse::<f64>().ok()),
    ) {
        // Extract OHLC values from the expected JSON object and convert them to f64
        if let (
            Some(expected_open),
            Some(expected_high),
            Some(expected_low),
            Some(expected_close),
        ) = (
            expected_json.get("open").and_then(|o| o.as_str()).and_then(|o| o.parse::<f64>().ok()),
            expected_json.get("high").and_then(|h| h.as_str()).and_then(|h| h.parse::<f64>().ok()),
            expected_json.get("low").and_then(|l| l.as_str()).and_then(|l| l.parse::<f64>().ok()),
            expected_json.get("close").and_then(|c| c.as_str()).and_then(|c| c.parse::<f64>().ok()),
        ) {
            // Compare the OHLC values using the specified precision
            (
                round_to(output_open, precision),
                round_to(output_high, precision),
                round_to(output_low, precision),
                round_to(output_close, precision),
            ) == (
                round_to(expected_open, precision),
                round_to(expected_high, precision),
                round_to(expected_low, precision),
                round_to(expected_close, precision),
            )
        } else {
            false
        }
    } else {
        false
    }
}

// Function to compare OHLC files and create a report
pub fn compare_files(output_path: &str, expected_path: &str, report_path: &str) {
    // Open the output file
    let output_file = File::open(output_path).expect("Failed to open output file");
    let output_reader = BufReader::new(output_file);

    // Open the expected output file
    let expected_file = File::open(expected_path).expect("Failed to open expected output file");
    let expected_reader = BufReader::new(expected_file);

    // Create the report file
    let mut report_file = File::create(report_path).expect("Failed to create report file");

    // Read the lines from both files
    let output_lines: Vec<String> = output_reader.lines().map(|line| line.unwrap()).collect();
    let expected_lines: Vec<String> = expected_reader.lines().map(|line| line.unwrap()).collect();

    // Compare the lines
    let mut num_matches = 0;
    let mut num_mismatches = 0;

    for (output_line, expected_line) in output_lines.iter().zip(expected_lines.iter()) {
        // Parse the JSON objects from the lines
        let output_json = serde_json::from_str::<serde_json::Value>(&output_line).expect("Failed to parse JSON");
        let expected_json = serde_json::from_str::<serde_json::Value>(&expected_line).expect("Failed to parse JSON");
        
        // Compare the JSON objects (including OHLC values)
        if compare_ohlc_values(&output_json, &expected_json, *MIN_DIGITS) {
            num_matches += 1;
        } else {
            num_mismatches += 1;
            // Write the mismatched lines to the report file
            writeln!(report_file, "Output: {},\nExpected: {}\n", output_line, expected_line)
                .expect("Failed to write to report file");
        }
    }

    let total_lines = output_lines.len();

    // Print the report
    println!("Report:");
    println!("Total lines: {}", total_lines);
    println!("Matching lines: {}", num_matches);
    println!("Mismatched lines: {}", num_mismatches);
}
