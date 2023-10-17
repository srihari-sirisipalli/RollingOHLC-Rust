# Tensorfox Homework Assignment Solution

## Task 1: Rust Library Crate for Rolling OHLC Calculation

### Overview
For Task 1, I have developed a Rust library crate named "rolling_ohlc" that provides a way to compute rolling OHLC (Open-High-Low-Close) values for a stream of numeric prices and timestamps within a given time window. The time window, which is a configurable parameter, enables us to calculate OHLC values over specified time intervals.

### Crate Structure
The crate is structured as follows:

- `lib.rs`: This file contains the core implementation of the rolling OHLC calculation and utility functions.
- `Cargo.toml`: This file defines the crate's metadata and dependencies.
- `tests`: The "tests" directory includes test modules to ensure the correctness of OHLC calculations.
- `examples`: The "examples" directory provides usage examples of the library.

### OHLC Calculation
The heart of the library is the `update_ohlc` function, which is responsible for updating OHLC data. This function accepts three parameters: a JSON string representing a ticker update, a mutable reference to a `HashMap` for storing symbol-specific OHLC data, and a time window in milliseconds. The `update_ohlc` function performs the following steps:

1. Parse the JSON object from the input line.
2. Extract the symbol and timestamp from the JSON object.
3. Get the previous OHLC data for the symbol from the HashMap or initialize it if it's the first data point.
4. Calculate the time difference between the current and previous timestamps.
5. Parse the bid and ask prices from the JSON object, calculate the current price, and round it to the specified number of decimal places.
6. Update the OHLC data based on the time window and previous data. If the time difference exceeds the window or it's the first data point, the OHLC data is updated accordingly.

## Task 2: Using the Library as a Dependency

### Overview
For Task 2, I have utilized the "rolling_ohlc" crate as a dependency to create an application that reads JSON data from a file and calculates rolling OHLC for multiple symbols. The time window is specified at startup and is consistent for all symbols.

### File Structure
- `main.rs`: This is the entry point of the application.
- `data`: The "data" directory contains the input data files and expected output data.
- `report`: The "report" directory is where comparison reports are generated during testing.

### OHLC Calculation Application
The application's `main.rs` file reads ticker updates from a specified file and calculates rolling OHLC for each symbol. It then outputs the results in the same format as the provided example (`data/dataset-a.txt`). The application uses the `compare_ohlc_values` function to compare OHLC values with a specified precision.

The flow of the application's code can be summarized as follows:

1. Open and read the input data file, which contains ticker updates for various symbols.
2. Initialize a HashMap to store OHLC data for each symbol.
3. Read each line (ticker update) from the input file.
4. For each line, parse the JSON object, extract the symbol and timestamp.
5. Calculate and update the rolling OHLC data using the "rolling_ohlc" library crate.
6. Compare the calculated OHLC values with the expected values (if available).
7. Write the calculated OHLC values to the output file in the specified format.
8. If any discrepancies are found, they are recorded in a report file for further analysis.

### Dataset
- The provided dataset files (`data/dataset-a.txt` and `data/dataset-b.txt`) represent ticker updates for various symbols.
- The time window for OHLC calculation is set to 5 minutes.

## Running the Solution
To run the solution, follow these steps:

1. Build the "rolling_ohlc" library crate using the command: `cargo build` in the root directory of the crate.
2. To run the OHLC calculation application for a specific data file, navigate to the root of the crate and use the following command: `cargo run -- [data_file]`
Replace `[data_file]` with the path to the dataset you want to process.
