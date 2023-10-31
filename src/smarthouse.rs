use crate::devices::*;
use crate::smartroom::*;
use std::collections::HashMap;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct SmartHouse {
    house_name: String,
    pub smart_rooms: HashMap<String, SmartRoom>,
}

impl Display for SmartHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "House name: {}", &self.house_name)
    }
}

impl SmartHouse {
    pub fn new(house_name: String) -> SmartHouse {
        SmartHouse {
            house_name,
            smart_rooms: HashMap::new(),
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
            None => None,
        }
    }

    pub fn create_report(&self, provider: impl DeviceInfoProvider) -> String {
        let mut report = String::new();
        report.push_str(&format!("{}", self));
        for smart_rooms in self.get_rooms() {
            report.push_str(&format!("{}", smart_rooms));
            for devices in smart_rooms.smart_device.values() {
                report.push_str(&format!("{}", provider.device_info(smart_rooms, devices)));
            }
        }
        report
    }
}

pub trait DeviceInfoProvider {
    fn device_info(&self, room: &SmartRoom, devices: &Device) -> String;
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
    fn device_info(&self, room: &SmartRoom, devices: &Device) -> String {
        let mut device_info = format!("{}", room.room_name);
        match devices {
            Device::SmartSocket(soket) => device_info.push_str(format!("{}", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}", thermo).as_str()),
        }
        device_info
    }
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room: &SmartRoom, devices: &Device) -> String {
        let mut device_info = format!("{}", room.room_name);
        match devices {
            Device::SmartSocket(soket) => device_info.push_str(format!("{}", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}", thermo).as_str()),
        }
        device_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_house() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let mut kitchen = SmartRoom::default("Kithen".to_string());
        kitchen.smart_device.insert(
            "Kitchen thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut hall = SmartRoom::default("Hall".to_string());
        hall.smart_device.insert(
            "Hall socket".to_string(),
            Device::SmartSocket(socket.clone()),
        );
        let mut bathroom = SmartRoom::default("Bathroom".to_string());
        bathroom.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut living = SmartRoom::default("Living room".to_string());
        living.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        living.smart_device.insert(
            "Bathroom socket".to_string(),
            Device::SmartSocket(socket.clone()),
        );
        let mut house = SmartHouse::new("House".to_string());
        house.smart_rooms.insert("Kitchen".to_string(), kitchen);
        assert_eq!(house.house_name, "House");
    }

    #[test]
    fn get_rooms() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let mut kitchen = SmartRoom::default("Kithen".to_string());
        kitchen.smart_device.insert(
            "Kitchen thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut hall = SmartRoom::default("Hall".to_string());
        hall.smart_device.insert(
            "Hall socket".to_string(),
            Device::SmartSocket(socket.clone()),
        );
        let mut bathroom = SmartRoom::default("Bathroom".to_string());
        bathroom.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut living = SmartRoom::default("Living room".to_string());
        living.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut house = SmartHouse::new("House".to_string());
        house.smart_rooms.insert("Kitchen".to_string(), kitchen);

        assert!(!house.get_rooms().is_empty())
    }

    #[test]

    fn create_report() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let socket_borrow = SmartSocket::default("Smart_socket borrow".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let mut kitchen = SmartRoom::default("Kithen".to_string());
        kitchen.smart_device.insert(
            "Kitchen thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut hall = SmartRoom::default("Hall".to_string());
        hall.smart_device.insert(
            "Hall socket".to_string(),
            Device::SmartSocket(socket.clone()),
        );
        let mut bathroom = SmartRoom::default("Bathroom".to_string());
        bathroom.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        let mut living = SmartRoom::default("Living room".to_string());
        living.smart_device.insert(
            "Bathroom thermo".to_string(),
            Device::SmartThermometr(thermo.clone()),
        );
        living.smart_device.insert(
            "Bathroom socket".to_string(),
            Device::SmartSocket(socket.clone()),
        );
        let mut house = SmartHouse::new("House".to_string());
        house.smart_rooms.insert("Kitchen".to_string(), kitchen);
        let info_provider_1 = OwningDeviceInfoProvider { socket: socket };

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket_borrow,
            thermo: &thermo,
        };

        assert!(!house.create_report(info_provider_1).is_empty());
        assert!(!house.create_report(info_provider_2).is_empty());
    }
}
