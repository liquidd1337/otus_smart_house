use std::collections::HashMap;

use crate::devices;
use crate::devices::*;
use crate::smartroom::*;
pub struct SmartHouse {
    house_name: String,
    smart_rooms: HashMap<String, SmartRoom>,
}

impl SmartHouse {
    pub fn new(house_name: String) -> Result<SmartHouse, &'static str> {
        match house_name {
            ref String => {
                 Ok (SmartHouse {
                    house_name,
                    smart_rooms: HashMap::new(),
                })
            },
            _ => Err("invalid house name"),
        }
    }



    pub fn get_rooms(&self) -> Vec<&SmartRoom> {
        self.smart_rooms.values().collect()

    }

    pub fn device_info(&self, room: String) -> Option<Vec<&Device>> {
            match self.smart_rooms.get(&room) {
                Some(room) => {
                    let device_info = room.smart_device.values().collect();
                    Some(device_info)
                }
                None => None
            }
    }


    pub fn create_report(&self, provider: impl DeviceInfoProvider) -> String {
        let mut providers = Vec::new();
        for i in self.get_rooms() {
            providers.push(provider.device_info(i));
        }
        return format!(
            "House name : {}, 
                contains rooms {:?}.
                    The rooms have:
                    {:#?}",
            self.house_name,
            self.get_rooms(),
            providers,
        );
    }
}

pub trait DeviceInfoProvider {
    fn device_info(&self, room: &SmartRoom, devices: Device) -> String;
}


#[derive(Debug)]
pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
#[derive(Debug)]
pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room: &SmartRoom, devices: Device) -> String {
        let mut device_info = format!("{}", room.room_name);
        match devices {
            Device::SmartSocket(soket)=> device_info.push_str(format!("{}", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}", thermo).as_str())
        }
        return device_info

    }
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room: &SmartRoom, devices: Device) -> String {
        let mut device_info = format!("{}", room.room_name);
        match devices {
            Device::SmartSocket(soket)=> device_info.push_str(format!("{}", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}", thermo).as_str())
        }
        return device_info
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_house() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let kitchen = SmartRoom::default("kitchen".to_string(), socket.clone(), thermo.clone());
        let hall = SmartRoom::default("hall".to_string(), socket.clone(), thermo.clone());
        let living = SmartRoom::default("living".to_string(), socket.clone(), thermo.clone());
        let bathroom = SmartRoom::default("bathroom".to_string(), socket.clone(), thermo.clone());
        let house = SmartHouse::new("house".to_string(), kitchen, hall, living, bathroom);

        assert_eq!(house.house_name, "house");
    }

    #[test]
    fn get_rooms() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let kitchen = SmartRoom::default("kitchen".to_string(), socket.clone(), thermo.clone());
        let hall = SmartRoom::default("hall".to_string(), socket.clone(), thermo.clone());
        let living = SmartRoom::default("living".to_string(), socket.clone(), thermo.clone());
        let bathroom = SmartRoom::default("bathroom".to_string(), socket.clone(), thermo.clone());
        let house = SmartHouse::new("house".to_string(), kitchen, hall, living, bathroom);

        assert_eq!(house.get_rooms(), vec![
            "kitchen".to_string(),
            "hall".to_string(),
            "living".to_string(),
            "bathroom".to_string()
        ])
    }

    #[test]

    fn create_report() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let socket_borrow = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let kitchen = SmartRoom::default("kitchen".to_string(), socket.clone(), thermo.clone());
        let hall = SmartRoom::default("hall".to_string(), socket.clone(), thermo.clone());
        let living = SmartRoom::default("living".to_string(), socket.clone(), thermo.clone());
        let bathroom = SmartRoom::default("bathroom".to_string(), socket.clone(), thermo.clone());
        let house = SmartHouse::new("house".to_string(), kitchen, hall, living, bathroom);

        let info_provider_1 = OwningDeviceInfoProvider { socket: socket };

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket_borrow,
            thermo: &thermo,
        };

        assert!(!house.create_report(info_provider_1).is_empty());
        assert!(!house.create_report(info_provider_2).is_empty());
    }
    


}