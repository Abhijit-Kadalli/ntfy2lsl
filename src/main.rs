use lsl::{self, Pushable, StreamInfo, StreamOutlet};
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::env;
use std::thread;
use std::time::Duration;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the token from environment variables
    let token = env::var("NTFY_API_TOKEN").expect("NTFY_API_TOKEN must be set");
    let ntfy_url = env::var("NTFY_URL").unwrap_or_else(|_| "https://ntfy.dognosis.link/oneapi/json".to_string());

    // Set up LSL stream and outlet
    let info = StreamInfo::new(
        "NtfyAlerts", "Notifications", 1, 0.0, 
        lsl::ChannelFormat::String, "ntfy_alerts_001")?;
    let outlet = StreamOutlet::new(&info, 0, 360)?;
    println!("LSL stream established.");

    // Create HTTP client
    let client = Client::new();
    let reconnect_delay = Duration::from_secs(10);

    println!("Attempting to connect to NTFY stream at: {}", ntfy_url);

    loop {
        let resp_result = client.get(&ntfy_url)
            .header("Authorization", format!("Bearer {}", token))
            .send();

        match resp_result {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("Successfully connected to NTFY stream. Reading notifications...");
                    // Stream the response
                    let reader = BufReader::new(resp);
                    let mut lines = reader.lines();

                    loop {
                        match lines.next() {
                            Some(Ok(line)) => {
                                if !line.is_empty() {
                                    // Parse JSON
                                    match serde_json::from_str::<Value>(&line) {
                                        Ok(json) => {
                                            println!("Received notification: {:?}", json);
                                            
                                            // Forward to LSL - using message/title as the data
                                            let message = if let Some(msg) = json.get("message").and_then(Value::as_str) {
                                                msg.to_string()
                                            } else if let Some(title) = json.get("title").and_then(Value::as_str) {
                                                title.to_string()
                                            } else {
                                                // Fallback to raw line if no message/title
                                                println!("Warning: Notification missing 'message' and 'title', forwarding raw line.");
                                                line.clone()
                                            };
                                            
                                            if let Err(e) = outlet.push_sample(&vec![message]) {
                                                eprintln!("Error pushing sample to LSL: {}. Attempting to continue...", e);
                                                // Decide if LSL errors should break the connection loop or just be logged
                                                // For now, we just log and continue reading from ntfy
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Error parsing JSON: {}. Line: '{}'", e, line);
                                            // Continue to next line even if parsing fails
                                        }
                                    }
                                }
                            }
                            Some(Err(e)) => {
                                // Error reading line from stream (e.g., connection closed)
                                eprintln!("Error reading from NTFY stream: {}. Attempting reconnect...", e);
                                break; // Break inner loop to trigger reconnection
                            }
                            None => {
                                // Stream ended normally (shouldn't happen with ntfy.sh streams unless server stops)
                                println!("NTFY stream ended. Attempting reconnect...");
                                break; // Break inner loop to trigger reconnection
                            }
                        }
                    }
                } else {
                    // HTTP error (e.g., 401 Unauthorized, 404 Not Found, 5xx Server Error)
                    eprintln!("Received non-success status from NTFY: {}. Attempting reconnect...", resp.status());
                    // Potentially read the body here for more error details if needed
                    // let error_body = resp.text().unwrap_or_else(|_| "Failed to read error body".to_string());
                    // eprintln!("Error body: {}", error_body);
                }
            }
            Err(e) => {
                // Network error during connect/send
                eprintln!("Error connecting to NTFY: {}. Attempting reconnect...", e);
            }
        }

        // Wait before retrying connection
        println!("Waiting {} seconds before reconnecting...", reconnect_delay.as_secs());
        thread::sleep(reconnect_delay);
    }

    // This part is now unreachable due to the infinite loop, but kept for structural integrity
    // In a real-world scenario with shutdown signals, the loop would be broken.
    // Ok(())
}