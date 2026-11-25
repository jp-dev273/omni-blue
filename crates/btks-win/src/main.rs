const BLUETOOTH_KEYS_PATH: &str = "SYSTEM\\CurrentControlSet\\Services\\BTHPORT\\Parameters\\Keys";

fn main() -> windows_registry::Result<()> {
    let devices_keys: windows_registry::Key = match windows_registry::LOCAL_MACHINE.open(BLUETOOTH_KEYS_PATH) {
        Ok(key) => key,
        Err(error) => {
            eprintln!("Failed to open Bluetooth keys: {}", error);
            return Err(error);
        }
    };

    let bluetooth_adapters_path = match devices_keys.keys() {
        Ok(keys) => keys,
        Err(error) => {
            eprintln!("Unable to list Bluetooth adapters: {}", error);
            return Err(error);
        }
    };

    for bluetooth_adapter_path in bluetooth_adapters_path {
        println!("Bluetooth Adapter: {}", bluetooth_adapter_path);

        let bluetooth_adapter_key = match devices_keys
            .open(&bluetooth_adapter_path) {
            Ok(key) => key,
            Err(error) => {
                eprintln!("Could not open Bluetooth Adapter {}: {}", bluetooth_adapter_path, error);
                continue;
            }
        };

        match bluetooth_adapter_key.values() {
            Ok(adapter_values) => {
                for adapter_value in adapter_values {
                    let value_name = adapter_value.0;

                    if detect_classic_devices(&value_name) {
                        println!("Classic device name: {}", value_name);
                        let classic_device_values = adapter_value.1.as_ref();
                        let classic_device_value_hex: String = classic_device_values
                            .iter().map(|byte| format!("{:02X}", byte)).collect();
                        println!("Classic device value: {}", classic_device_value_hex);
                    } else {
                        println!("Extra record name: {}", value_name);
                        println!("Extra record value: {:?}", adapter_value.1);
                    }
                };
            },
            Err(error) => eprintln!("Could not open Bluetooth Adapter values {}: {}", bluetooth_adapter_path, error)
        };

        match bluetooth_adapter_key.keys() {
            Ok(adapter_sub_keys) => {
                for adapter_sub_key_path in adapter_sub_keys {
                    println!("BLE Device key: {}", adapter_sub_key_path);

                    let ble_bluetooth_device_key = match bluetooth_adapter_key.open(&adapter_sub_key_path)  {
                        Ok(key) => key,
                        Err(error) => {
                            eprintln!("Could not open BLE Device Key {}: {}", adapter_sub_key_path, error);
                            continue;
                        }
                    };

                    let ble_bluetooth_device_values = match ble_bluetooth_device_key.values()  {
                        Ok(values) => values,
                        Err(error) => {
                            eprintln!("Could not open BLE Bluetooth Device value on {}: {}", adapter_sub_key_path, error);
                            continue;
                        }
                    };

                    for ble_bluetooth_device_value in ble_bluetooth_device_values {
                        let value_name = ble_bluetooth_device_value.0;
                        println!("BLE field: {}", value_name);

                        println!("BLE value: {:?}", ble_bluetooth_device_value.1);
                    }
                }
            },
            Err(error) => eprintln!("Could not access BLE Device Keys on adapter {}: {}", bluetooth_adapter_path, error)
        };
    }

    Ok(())
}

fn detect_classic_devices(value: &String) -> bool {
    let trimmed = if value.starts_with("0x") { &value[2..] } else { &value[..] };
    const EXACT_KEY_NAME_LENGTH: u8 = 12;
    if trimmed.len() != EXACT_KEY_NAME_LENGTH as usize {
        return false;
    };
    if !trimmed.chars().all(|char| char.is_ascii_hexdigit()) {
        return false;
    };
    return true;
}
