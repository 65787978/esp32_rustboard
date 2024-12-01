# Rustboard Keyboard Firmware

## Introduction

Welcome to the Rustboard Keyboard Firmware project! This firmware is designed from the ground up for the ESP32C3 microcontrollers, providing a robust solution for split keyboard layouts. It leverages Bluetooth Low Energy (BLE) for seamless wireless connectivity and is developed using the Rust programming language, ensuring safety and performance.

This project aims to provide a customizable and efficient firmware solution for enthusiasts and developers looking to create their own split keyboard setups. With a focus on modularity and ease of use, you can easily adapt the firmware to suit your specific needs.

## Current Bugs

- **The key 'A' is not being recognized by the OS**: The keycode for the 'A' character is not being recognized by the OS.
- **Modifier keys are not working**: The current implementation of the sending logic is needs to be improved.
- **Both halves of the keyboard are connected individually**: As of now, both keyboard halves are connected as indipendant keyboards. This will be fixed in the future.

## How to Build

To compile the firmware, follow these steps:

1. **Install Rust**: Ensure you have Rust installed on your system. You can install it using [rustup](https://rustup.rs/).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/65787978/esp32_rustboard.git
   cd esp32_rustboard
   ```

3. **Set Up the ESP32 Rust Toolchain**: Follow the instructions in the [ESP-IDF documentation GitHub](https://github.com/esp-rs) to set up the ESP32 Rust toolchain.

4. **Build the Firmware**:
     For left half of the keyboard:
   ```bash
   cargo build --release --features "left-side"
   ```
   
      For right half of the keyboard:
   ```bash
   cargo build --release --features "right-side"
   ```

6. **Flash the Firmware**: Connect your ESP32C3 device and use the following command to flash the firmware:
   ```bash
   espflash flash ./target/riscv32imc-esp-espidf/release/esp32_rustboard --monitor
   ```

## Contributing

We welcome contributions! If you would like to contribute to the project, please fork the repository and submit a pull request. For any questions or discussions, feel free to open an issue.
