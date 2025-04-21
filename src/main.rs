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
    let ntfy_url = env::var("NTFY_URL").unwrap_or_else(|_| "https://ntfy.sh/diskalerts/sse".to_string());

    // Set up LSL stream and outlet
    let info = StreamInfo::new(
        "NtfyAlerts", "Notifications", 1, 0.0, 
        lsl::ChannelFormat::String, "ntfy_alerts_001")?;
    let outlet = StreamOutlet::new(&info, 0, 360)?;
    println!("LSL stream established.");

    // Create HTTP client with no timeout for persistent SSE connection
    let client = Client::builder()
        .timeout(None) // Disable request/response timeout
        .build()?;
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
                    let reader = BufReader::new(resp);
                    let mut lines = reader.lines();

                    loop {
                        match lines.next() {
                            Some(Ok(line)) => {
                                // SSE messages are typically prefixed with 'data: '
                                if line.starts_with("data: ") {
                                    let json_str = &line["data: ".len()..];
                                    if !json_str.is_empty() {
                                        // Parse JSON payload from the 'data' field
                                        match serde_json::from_str::<Value>(json_str) {
                                            Ok(json) => {
                                                // Check if it's a message event (ignore open/keepalive)
                                                if let Some(event_type) = json.get("event").and_then(Value::as_str) {
                                                    if event_type == "message" {
                                                        println!("Received notification: {:?}", json);

                                                        // Forward to LSL - using message/title as the data
                                                        let message = if let Some(msg) = json.get("message").and_then(Value::as_str) {
                                                            msg.to_string()
                                                        } else if let Some(title) = json.get("title").and_then(Value::as_str) {
                                                            title.to_string()
                                                        } else {
                                                            // Fallback if no message/title in a message event
                                                            println!("Warning: Message event missing 'message' and 'title', forwarding raw JSON data.");
                                                            json_str.to_string()
                                                        };

                                                        if let Err(e) = outlet.push_sample(&vec![message]) {
                                                            eprintln!("Error pushing sample to LSL: {}. Attempting to continue...", e);
                                                            // LSL errors currently don't break the ntfy connection loop
                                                        }
                                                    } else {
                                                        // Optionally log other event types like 'open' or 'keepalive'
                                                         println!("Received SSE event (ignored): {} ({})", event_type, json_str);
                                                    }
                                                } else {
                                                    // Handle cases where JSON is valid but missing the 'event' field
                                                     println!("Warning: Received SSE data missing 'event' field: {}", json_str);
                                                }
                                            }
                                            Err(e) => {
                                                eprintln!("Error parsing JSON from SSE data: {}. Data: '{}'", e, json_str);
                                                // Continue to next line even if parsing fails
                                            }
                                        }
                                    }
                                } else if !line.is_empty() {
                                    // Log other non-empty lines (e.g., 'event: open', 'event: keepalive', blank lines) if needed
                                     println!("Received non-data line (ignored): {}", line);
                                }
                                // Ignore empty lines often sent as part of SSE protocol
                            }
                            Some(Err(e)) => {
                                // Error reading line from stream (e.g., connection closed unexpectedly)
                                eprintln!("Error reading from NTFY stream: {}. Attempting reconnect...", e);
                                break; // Break inner loop to trigger reconnection
                            }
                            None => {
                                // Stream ended gracefully by the server (might happen occasionally)
                                println!("NTFY stream ended. Attempting reconnect...");
                                break; // Break inner loop to trigger reconnection
                            }
                        }
                    }
                } else {
                    // HTTP error (e.g., 401 Unauthorized, 404 Not Found, 5xx Server Error)
                    eprintln!("Received non-success status from NTFY: {}. Attempting reconnect...", resp.status());
                    // Optionally read the body for more details:
                    // let error_body = resp.text().unwrap_or_else(|_| "Failed to read error body".to_string());
                    // eprintln!("Error details: {}", error_body);
                }
            }
            Err(e) => {
                // Network error during initial connect or sending request
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