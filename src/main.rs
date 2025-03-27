use lsl::{self, Pushable, StreamInfo, StreamOutlet};
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the token from environment variables
    let token = env::var("NTFY_API_TOKEN").expect("NTFY_API_TOKEN must be set");

    // Set up LSL stream and outlet
    let info = StreamInfo::new(
        "NtfyAlerts", "Notifications", 1, 0.0, 
        lsl::ChannelFormat::String, "ntfy_alerts_001")?;
    let outlet = StreamOutlet::new(&info, 0, 360)?;

    // Create HTTP client
    let client = Client::new();
    let resp = client.get("https://ntfy.sh/diskalerts/json")
        .header("Authorization", format!("Bearer {}", token))
        .send()?;
    
    // Stream the response
    let reader = BufReader::new(resp);
    for line in reader.lines() {
        if let Ok(line) = line {
            if !line.is_empty() {
                // Parse JSON
                if let Ok(json) = serde_json::from_str::<Value>(&line) {
                    println!("Received notification: {:?}", json);
                    
                    // Forward to LSL - using message/title as the data
                    let message = if let Some(msg) = json.get("message").and_then(Value::as_str) {
                        msg.to_string()
                    } else if let Some(title) = json.get("title").and_then(Value::as_str) {
                        title.to_string()
                    } else {
                        line.clone()
                    };
                    
                    outlet.push_sample(&vec![message])?;
                }
            }
        }
    }

    Ok(())
}