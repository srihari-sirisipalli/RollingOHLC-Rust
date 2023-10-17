use std::collections::HashMap;
use rolling_ohlc::update_ohlc;


fn main() {
    let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();

    // Example input line
    let line = r#"{"s":"TURBOUSDT","b":"0.3261","a":"0.3262","T":1662022800005}"#;

    // Update OHLC data
    if let Err(err) = update_ohlc(line, &mut symbol_data, 300000) {
        eprintln!("Failed to update OHLC data: {}", err);
    }

    // Print the updated symbol data
    for (symbol, data) in symbol_data.iter() {
        println!("Symbol: {}", symbol);
        println!("Timestamp: {}", data.0);
        println!("Open: {:?}", data.1);
        println!("High: {:?}", data.2);
        println!("Low: {:?}", data.3);
        println!("Close: {:?}", data.4);
        println!("-----------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_ohlc() {
        let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();
        let _ = update_ohlc(r#"{"s":"TURBOUSDT","b":"0.3261","a":"0.3262","T":1662022800005}"#, &mut symbol_data, 300000);

        let expected_result = (
            1662022800005,
            Some(0.326150),
            Some(0.326150),
            Some(0.326150),
            Some(0.326150),
        );
        assert_eq!(symbol_data.get("TURBOUSDT"), Some(&expected_result));
    }


    #[test]
    fn test_update_ohlc_missing_fields() {
        let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();
        
        // Missing required fields: "s", "b", "a", "T"
        assert!(update_ohlc(r#"{"e":"bookTicker","u":1875301601431}"#, &mut symbol_data, 300000).is_err());
    }


    #[test]
    fn test_update_ohlc_invalid_price() {
        let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();
        
        // Invalid price: "b" contains an invalid float value
        assert!(update_ohlc(r#"{"e":"bookTicker","u":1875301601431,"s":"FISHUSDT","b":"invalid","B":"2620","a":"0.13379","A":"3000","T":1662022800574,"E":1662022800578}"#, &mut symbol_data, 300000).is_err());
    }

    #[test]
    fn test_update_ohlc_missing_symbol() {
        let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();
        
        // Missing symbol field
        assert!(update_ohlc(r#"{"e":"bookTicker","u":1875301601431,"b":"0.13378","B":"2620","a":"0.13379","A":"3000","T":1662022800574,"E":1662022800578}"#, &mut symbol_data, 300000).is_err());
    }

    #[test]
    fn test_update_ohlc_invalid_timestamp() {
        let mut symbol_data: HashMap<String, (u64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)> = HashMap::new();
        
        // Invalid timestamp: "T" field contains a negative value
        assert!(update_ohlc(r#"{"e":"bookTicker","u":1875301601431,"s":"FISHUSDT","b":"0.13378","B":"2620","a":"0.13379","A":"3000","T":-1662022800574,"E":1662022800578}"#, &mut symbol_data, 300000).is_err());
    }


    
}
