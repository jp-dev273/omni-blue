const BLUETOOTH_KEYS_PATH: &str = "SYSTEM\\CurrentControlSet\\Services\\BTHPORT\\Parameters\\Keys";

fn main() {
    let devices_keys: windows_registry::Key = windows_registry::LOCAL_MACHINE.open(BLUETOOTH_KEYS_PATH).unwrap();

    let bluetooth_adapters = devices_keys.keys().expect("Did not find Bluetooth Adapters");

    bluetooth_adapters.for_each(|bluetooth_adapter| {
        println!("Bluetooth Adapter: {}", bluetooth_adapter);
        let adapter_key = devices_keys
            .open(&bluetooth_adapter).unwrap();

        let adapter_values = adapter_key.values().expect("Did not find Classic Devices");

        adapter_values.for_each(|adapter_value| {
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
        })

    });
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
