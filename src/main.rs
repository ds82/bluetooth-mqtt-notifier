// See the "macOS permissions note" in README.md before running this on macOS
// Big Sur or later.

use mqtt::{AsyncClient, CreateOptions};
use regex::Regex;
use std::borrow::Borrow;
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Manager, PeripheralId};

use futures::executor::block_on;
use paho_mqtt as mqtt;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    mqtt_connect();
    scan_bluetooth().await?;

    Ok(())
}

async fn scan_bluetooth() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let clean_uuid_re = Regex::new(r"hci[0-9]*\/dev_|_").unwrap();
    let search_for_uuids_env = env::var("SEARCH_UUIDS").unwrap_or(String::from(""));
    let search_for_uuids = search_for_uuids_env.split(",").collect::<Vec<&str>>();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan on {}...", adapter.adapter_info().await?);
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(10)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;

                let id = format!("{}", peripheral.id()).as_str().to_lowercase();
                let formated_id = clean_uuid_re.replace_all(id.as_str(), "");
                let should_publish = search_for_uuids.contains(&formated_id.as_ref());

                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));

                println!(
                    "found {} [{}] --> {:?}",
                    formated_id, local_name, should_publish
                );
            }
        }
    }
    Ok(())
}

fn mqtt_connect() -> () {
    // Command-line option(s)
    let host = env::var("MQTT_HOST").unwrap_or("mqtt://localhost:1883".to_string());
    let user_name = env::var("MQTT_USER").unwrap_or("".to_string());
    let password = env::var("MQTT_PASSWORD").unwrap_or("".to_string());

    let create_options = mqtt::CreateOptionsBuilder::new().finalize();
    let connect_options = mqtt::ConnectOptionsBuilder::new()
        .user_name(user_name)
        .password(password)
        .server_uris(&[host])
        .finalize();

    // Create the client
    let cli = mqtt::AsyncClient::new(create_options).unwrap_or_else(|err| {
        println!("Error creating the client: {}", err);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // Connect with default options and wait for it to complete or fail
        // The default is an MQTT v3.x connection.
        println!("Connecting to the MQTT server");
        cli.connect(connect_options).await?;

        // Create a message and publish it
        println!("Publishing a message on the topic 'test'");
        let msg = mqtt::Message::new("test", "Hello Rust MQTT world!", mqtt::QOS_1);
        cli.publish(msg).await?;

        // Disconnect from the broker
        println!("Disconnecting");
        cli.disconnect(None).await?;

        // Ok::<(), mqtt::Error>(())
        Ok::<AsyncClient, mqtt::Error>(cli)
    }) {
        eprintln!("{}", err);
    }
}
