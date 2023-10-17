Flow of the Program:

1. Read Input Data:

	-The program starts by reading input data from a specified file (e.g., "data/input/dataset-a.txt").
	-The input file contains JSON objects representing financial market data for different symbols.

2. Initialize OHLC Data Storage:
	
	-The program creates an empty HashMap called "symbol_data" to store OHLC data for each symbol.
	-The HashMap will use the symbol as the key and a tuple containing OHLC data (timestamp, open, high, low, close) as the value.
	
3. Process Each Line:

	-The program iterates through each line of the input file.
	-For each line, it processes the JSON object contained in that line.

4. Extract Symbol and Timestamp:

	-From the JSON object, the program extracts the "symbol" and "timestamp" fields.
	-The symbol represents the trading symbol for which OHLC data needs to be updated.
	-The timestamp indicates the time at which the market data was recorded.

5. Calculate Current Price:

	-The program extracts the "bid" and "ask" prices from the JSON object.
	-It calculates the current price as the average of the bid and ask prices.
	-The current price is used to update the OHLC data.

6. Update OHLC Data:

	-The program checks if OHLC data exists for the current symbol in the HashMap "symbol_data."
	-If OHLC data does not exist or if the time difference between the current and previous timestamps is greater than the specified time window 	(e.g., 300,000 milliseconds or 5 minutes), new OHLC data is created for the symbol.
	-If OHLC data already exists and the time difference is within the time window, the OHLC data is updated accordingly.

7. Round OHLC Values:

	-The OHLC values are rounded to a specified number of decimal places using the "round_to" function from the "rolling_ohlc" library.
	-The specified number of decimal places is represented by the global constant "MIN_DIGITS."

8. Write to Output File:

	-After processing all lines of the input file, the program generates an output file to store the updated OHLC data.
	-The OHLC data for each symbol is formatted into a JSON object and written to the output file.
	-The output file path is specified as "data/output/ohlc-5m-a_output.txt."

9. Compare with Expected Output:

	-Additionally, the program includes functionality for comparing the generated output file with an expected output file (e.g., "data/expected/ohlc-5m-a.txt").
	-The program reads both output files and parses the JSON objects.
	-It compares the OHLC values for each symbol and identifies any mismatches.
	-The mismatches are recorded in a report file (e.g., "data/report/a_report.txt").
