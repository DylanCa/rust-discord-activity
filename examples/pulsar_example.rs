/// Example simulating Pulsar Game Engine Rich Presence
use rust_discord_activity::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

fn main() {
    println!("🚀 Pulsar Game Engine - Discord Rich Presence Example");
    println!("====================================================\n");

    // IMPORTANT: Replace this with your Discord Application ID
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

    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // Simulate Pulsar Engine workflow
    let workflow = vec![
        ("Editing in Script Editor", "Project: SpaceGame | player.rs"),
        ("Editing in Level Editor", "Project: SpaceGame | level_01.scene"),
        ("Editing in DAW", "Project: SpaceGame | music_theme.pdaw"),
        ("Editing in Blueprint Editor", "Project: SpaceGame | PlayerController.class"),
        ("Editing in Script Editor", "Project: SpaceGame | enemy_ai.rs"),
    ];

    println!("🎨 Simulating Pulsar Engine workflow...");
    println!("   Each state lasts 8 seconds\n");

    for (i, (state, details)) in workflow.iter().enumerate() {
        println!("📝 Step {}/{}: {}", i + 1, workflow.len(), state);
        println!("   Details: {}", details);
        
        let timestamp = Timestamp::new(Some(start_time), None);
        let mut activity = Activity::new();
        
        activity
            .set_state(Some(state.to_string()))
            .set_details(Some(details.to_string()))
            .set_timestamps(Some(timestamp))
            .set_activity_type(Some(ActivityType::GAME));

        let payload = Payload::new(EventName::Activity, EventData::Activity(activity));

        match client.send_payload(payload) {
            Ok(_) => println!("   ✅ Presence updated!"),
            Err(e) => eprintln!("   ❌ Failed to update: {:?}", e),
        }

        // Wait 8 seconds
        for j in (1..=8).rev() {
            print!("\r   Next update in: {} seconds  ", j);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!();
    }

    println!("\n🎉 Pulsar Engine workflow simulation complete!");
    println!("💡 This is how Discord Rich Presence works in the actual engine!");
    println!("👋 Disconnecting...");
}
