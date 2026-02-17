# Tapo Scanner 

Rust-based Bluetooth Low Energy (BLE) device scanner and fingerprinting tool
tapo-scan is a lightweight Rust tool for scanning nearby BLE devices and inspecting their broadcast metadata, including manufacturer data and service payloads. It is designed to help explore, debug, and reverse-engineer BLE-based IoT and sensor devices.

# Why I Built This

When working with wearable and IoT devices, the first challenge is often understanding what data a device is broadcasting and how it identifies itself.
This tool was created to:

Inspect unknown BLE devices

Explore manufacturer-specific data

Debug BLE discovery pipelines

Learn how consumer sensors expose data at the protocol level

It is especially useful when dealing with devices that do not have public APIs or documentation.

Features

Scan for nearby BLE devices

Print device name and MAC address

Dump manufacturer-specific data

Dump service UUID payloads

Async scanning with Tokio

Cross-platform BLE support via btleplug
