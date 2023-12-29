# Rust Discord Activity
_A lightweight library developed in Rust to control Discord Rich Presence_

## How-to
Three steps:
1. Create your Activity and set desired data using provided structs 
2. Create a new Payload with your Activity
3. Instantiate a new DiscordClient and send your Payload through it

Et voilà !

## Example
```rust
let mut activity = Activity::new();

let limg = Some(String::from("https://placehold.co/600x400/png"));
let simg = Some(String::from("https://placehold.co/600x400/png"));
let asset = Asset::new(limg, None, simg, None);
let now_in_millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
let timestamp = Timestamp::new(Some(now_in_millis - 10000), None);

let party = Party::new(None, Some((2, 4)));
let mut button_vec = vec![];
button_vec.push(Button::new("First Button".into(), "https://google.com".into()));
button_vec.push(Button::new("Second Button".into(), "https://yahoo.com".into()));

activity
.set_state(Some("This is State".into()))
.set_activity_type(Some(ActivityType::LISTENING))
.set_details(Some("This is Details".parse().unwrap()))
.set_timestamps(Some(timestamp))
.set_assets(Some(asset))
.set_party(Some(party))
.set_instance(Some(true))
.set_buttons(Some(button_vec));

let payload = Payload::new(EventName::Activity, EventData::Activity(activity));

let mut client = DiscordClient::new("<application_id>");
let _ = client.send_payload(payload);

```

And voilà! This sets-up a new Activity for the current Discord user:
![Discord Rich Presence](https://imgur.com/gf9pOen.png)

## Next Steps
- Write the documentation for this library
- Write unit tests
- Push the library on crates.io