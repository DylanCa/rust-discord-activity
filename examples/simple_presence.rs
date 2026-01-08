use rust_discord_activity::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🎮 Discord Rich Presence Example");
    println!("================================\n");

    // IMPORTANT: Replace this with your Discord Application ID
    // Get one from: https://discord.com/developers/applications
    let application_id = "YOUR_DISCORD_APPLICATION_ID_HERE";
    
    if application_id == "YOUR_DISCORD_APPLICATION_ID_HERE" {
        eprintln!("❌ ERROR: Please replace YOUR_DISCORD_APPLICATION_ID_HERE with your actual Discord Application ID");
        eprintln!("   Get one from: https://discord.com/developers/applications");
        std::process::exit(1);
    }

    println!("📡 Connecting to Discord...");
    let mut client = DiscordClient::new(application_id);

    match client.connect() {
        Ok(_) => {
            println!("✅ Connected to Discord!\n");
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to Discord: {:?}", e);
            eprintln!("   Make sure Discord is running!");
            std::process::exit(1);
        }
    }

    // Get current timestamp for "elapsed time"
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    println!("📝 Setting up Rich Presence activity...");
    
    // Create timestamp to show elapsed time
    let timestamp = Timestamp::new(Some(start_time), None);

    // Create the activity
    let mut activity = Activity::new();
    activity
        .set_state(Some("Testing Discord RPC".into()))
        .set_details(Some("Cross-platform Discord integration".into()))
        .set_timestamps(Some(timestamp))
        .set_activity_type(Some(ActivityType::GAME));

    // Optional: Add custom assets (you need to upload these in Discord Developer Portal)
    // let asset = Asset::new(
    //     Some("large_image_key".into()),  // Upload image in Discord Dev Portal
    //     Some("Large Image Hover Text".into()),
    //     Some("small_image_key".into()),
    //     Some("Small Image Hover Text".into()),
    // );
    // activity.set_assets(Some(asset));

    // Optional: Add buttons (max 2)
    // let mut buttons = vec![];
    // buttons.push(Button::new("GitHub".into(), "https://github.com".into()));
    // buttons.push(Button::new("Website".into(), "https://example.com".into()));
    // activity.set_buttons(Some(buttons));

    // Create and send the payload
    let payload = Payload::new(EventName::Activity, EventData::Activity(activity));

    match client.send_payload(payload) {
        Ok(_) => {
            println!("✅ Rich Presence updated successfully!\n");
            println!("🎉 Check your Discord profile - you should see the activity!");
            println!("   State: Testing Discord RPC");
            println!("   Details: Cross-platform Discord integration");
            println!("   Timestamp: Elapsed time since now");
        }
        Err(e) => {
            eprintln!("❌ Failed to send payload: {:?}", e);
            std::process::exit(1);
        }
    }

    println!("\n⏱️  Keeping presence active for 30 seconds...");
    println!("   (Press Ctrl+C to exit early)");
    
    // Keep the program running to maintain the presence
    for i in (1..=30).rev() {
        print!("\r   Time remaining: {} seconds  ", i);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n\n👋 Example complete! Disconnecting...");
    println!("   (Discord presence will clear in a few seconds)");
}
