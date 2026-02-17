use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.into_iter().next().expect("No Bluetooth adapters found");

    println!("Starting scan for 5 seconds...");
    adapter.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(5)).await;

    let peripherals = adapter.peripherals().await?;
    for p in peripherals {
        if let Ok(Some(props)) = p.properties().await {
            println!("Name: {:?}", props.local_name);
            println!("Address: {:?}", p.address());
            if let Some(md) = props.manufacturer_data {
                println!("Manufacturer data:");
                for (k, v) in md {
                    println!("  ID 0x{:04x}: {:?}", k, v);
                }
            }
            if let Some(svc_data) = props.service_data {
                println!("Service data:");
                for (uuid, data) in svc_data {
                    println!("  {}: {:?}", uuid, data);
                }
            }
            println!("---");
        }
    }

    adapter.stop_scan().await?;
    Ok(())
}