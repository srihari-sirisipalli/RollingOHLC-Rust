use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde_json;
use rolling_ohlc::update_ohlc;
// Uncomment below line for comparing
use rolling_ohlc::compare_files; // Import the compare_files function from lib

fn main() {
    // Create an empty dictionary to store symbol data
    let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();

    // Read the input file (Change "data/dataset-a.txt" to your desired input file path)
    let file = File::open("data/input/dataset-a.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Create the output file  ** Careful with file names
    let mut output_file = File::create("data/output/ohlc-5m-a_output.txt").expect("Failed to create output file");

    // Process each line in the file
    for line in reader.lines() {
        if let Ok(line) = line {
            // Process the line to update OHLC values in the dictionary
            let _ = update_ohlc(&line, &mut symbol_data, 300000);

            // Parse the line as JSON
            let json = serde_json::from_str::<serde_json::Value>(&line).expect("Failed to parse JSON");

            // Extract the symbol from the JSON object
            let symbol = json.get("s").and_then(|s| s.as_str()).unwrap_or("");

            // Check if the symbol exists in the symbol_data
            if let Some(symbol_values) = symbol_data.get(&symbol.to_string()) {
                // Create a string with the desired output format
                let output_string = format!(
                    "{{\"symbol\":\"{}\",\"timestamp\":{},\"open\":\"{}\",\"high\":\"{}\",\"low\":\"{}\",\"close\":\"{}\"}}",
                    symbol,
                    symbol_values.0,
                    symbol_values.1.unwrap_or_default(),
                    symbol_values.2.unwrap_or_default(),
                    symbol_values.3.unwrap_or_default(),
                    symbol_values.4.unwrap_or_default()
                );
                    
                // Write the string to the output file
                writeln!(output_file, "{}", output_string).expect("Failed to write to output file");
            }
        }
    }


    // UNCOMMENT BELOW LINE FOR Creating reports and comparing
    // Change file names
    compare_files("data/output/ohlc-5m-a_output.txt", "data/expected/ohlc-5m-a.txt", "data/report/a_report.txt"); // Call compare_files with appropriate file paths
}
