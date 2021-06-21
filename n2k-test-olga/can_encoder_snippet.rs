fn bloop() {
    match result {
        Ok(pgn) => {
            match pgn {
                Some(BatteryStatus(f)) if f.voltage().is_some() => {
                    log::info!("{} {:?}", "BatteryStatus", f);
                    cx.resources.boat_status.lock(|characteristic| {
                        characteristic.set(BoatStatus {
                            battery0_voltage: f.voltage().unwrap(),
                            battery1_voltage: characteristic.value().unwrap().battery1_voltage,
                            temperature: characteristic.value().unwrap().temperature,
                        });
                        publish_all::spawn().unwrap();
                    })
                }
                Some(BatteryStatus(f)) if f.voltage().is_some() => {
                    log::info!("{} {:?}", "BatteryStatus", f);
                    cx.resources.boat_status.lock(|characteristic| {
                        characteristic.set(BoatStatus {
                            battery0_voltage: characteristic.value().unwrap().battery0_voltage,
                            battery1_voltage: f.voltage().unwrap(),
                            temperature: characteristic.value().unwrap().temperature,
                        });
                        publish_all::spawn().unwrap();
                    })
                }
                Some(Temperature(f)) if f.actual_temperature().is_some() => {
                    log::info!("{} {:?}", "Temperature", f);
                    cx.resources.boat_status.lock(|characteristic| {
                        characteristic.set(BoatStatus {
                            battery0_voltage: characteristic.value().unwrap().battery0_voltage,
                            battery1_voltage: characteristic.value().unwrap().battery1_voltage,
                            temperature: f.actual_temperature().unwrap(),
                        });
                        publish_all::spawn().unwrap();
                    })
                }
                Some(pgn) => log::warn!("Unknown PGN: {:?}", pgn),
                None => {
                    log::warn!("No PGN supplied from CAN.");
                }
            };
        }
        Err(err) => {
            error!("Error when parsing PGN from CAN: {:?}", err);
        }
    };
}
