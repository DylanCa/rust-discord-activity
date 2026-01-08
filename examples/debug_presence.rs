/// Debug example to test Discord Rich Presence visibility
use rust_discord_activity::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🔍 Discord Rich Presence Debug Example");
    println!("=======================================\n");

    let application_id = "YOUR_DISCORD_APPLICATION_ID_HERE";
    
    if application_id == "YOUR_DISCORD_APPLICATION_ID_HERE" {
        eprintln!("❌ ERROR: Please replace YOUR_DISCORD_APPLICATION_ID_HERE with your actual Discord Application ID");
        std::process::exit(1);
    }

    println!("📡 Connecting to Discord...");
    let mut client = DiscordClient::new(application_id);

    match client.connect() {
        Ok(_) => {
            println!("✅ Connected to Discord!");
            println!("   Application ID: {}\n", application_id);
        }
        Err(e) => {
            eprintln!("❌ Failed to connect: {:?}", e);
            std::process::exit(1);
        }
    }

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("📝 Creating Rich Presence activity...");
    println!("   Start Time: {}\n", start_time);
    
    let timestamp = Timestamp::new(Some(start_time), None);

    // Create a very obvious activity
    let mut activity = Activity::new();
    activity
        .set_state(Some("🎮 TESTING DISCORD RPC".into()))
        .set_details(Some("If you see this, it works!".into()))
        .set_timestamps(Some(timestamp))
        .set_activity_type(Some(ActivityType::GAME));

    let payload = Payload::new(EventName::Activity, EventData::Activity(activity));

    println!("📤 Sending payload to Discord...");
    match client.send_payload(payload) {
        Ok((opcode, response)) => {
            println!("✅ Payload sent successfully!");
            println!("   OpCode: {}", opcode);
            println!("   Response: {:#?}\n", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to send payload: {:?}", e);
            std::process::exit(1);
        }
    }

    println!("⚠️  IMPORTANT: Check your Discord settings!");
    println!("   1. Open Discord Settings (gear icon)");
    println!("   2. Go to 'Activity Privacy'");
    println!("   3. Make sure 'Display current activity as a status message' is ENABLED");
    println!("   4. Also check 'Activity Status' → Make sure the toggle is ON\n");
    
    println!("👀 Where to look:");
    println!("   - Your own profile (click your avatar in bottom left)");
    println!("   - Your status in server member list");
    println!("   - Your profile when someone else views it\n");

    println!("⏱️  Keeping presence active for 60 seconds...");
    println!("   The presence should appear within 5-10 seconds");
    println!("   (Press Ctrl+C to exit early)\n");
    
    for i in (1..=60).rev() {
        if i % 10 == 0 {
            println!("   Still active... {} seconds remaining", i);
        }
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n👋 Example complete!");
}
