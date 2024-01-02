use thiserror::Error;
use crate::devices::*;
use crate::smartroom::*;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Error)]
pub enum SmartHouseError {
    #[error("Error while adding smart room")]
    AddRoomError(String),
    #[error("Error while removing smart room")]
    RemoveRoomError(String),
}
#[derive(Debug, Clone, Error)]
pub struct SmartHouse {
    house_name: String,
    pub smart_rooms: HashMap<String, SmartRoom>,
}

impl Display for SmartHouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "House name: {}\n", &self.house_name)
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

    pub fn add_smart_room(&mut self, room: SmartRoom) -> Result<(), SmartHouseError> {
        if let Ok(_room_name) = room.get_room_name() {
        self.smart_rooms.insert(room.get_room_name().unwrap(), room);
        Ok(())
        } else {
            Err(SmartHouseError::AddRoomError("Invalid room name".to_string()))
        }
    }

    pub fn remove_smart_room(&mut self, room: SmartRoom) -> Result<(), SmartHouseError> {
        if let Ok(_room_name) = room.get_room_name() {
            self.smart_rooms.remove(&room.room_name);
            Ok(())
        } else {
            Err(SmartHouseError::RemoveRoomError("There is no such room".to_string()))
        }
        
    }

    pub fn create_report(&self, provider: impl DeviceInfoProvider) -> String {
        let mut report = String::new();
        report.push_str(&format!("{}", self));
        for smart_rooms in self.get_rooms() {
            report.push_str(&format!("{} contains:\n", smart_rooms));
            for devices in smart_rooms.smart_device.values() {
                report.push_str(&format!("{}\n", provider.device_info(smart_rooms, devices)));
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
        let mut device_info = room.room_name.to_string();
        match devices {
            Device::SmartSocket(soket) => device_info.push_str(format!("{}", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}", thermo).as_str()),
        }
        device_info
    }
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room: &SmartRoom, devices: &Device) -> String {
        let mut device_info = room.room_name.to_string();
        match devices {
            Device::SmartSocket(soket) => device_info.push_str(format!("{}\n", soket).as_str()),
            Device::SmartThermometr(thermo) => device_info.push_str(format!("{}\n", thermo).as_str()),
        }
        device_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_house() {
        let house = SmartHouse::new("House".to_string());
        assert_eq!(house.house_name, "House");
    }

    #[test]
    fn add_smart_room() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let mut kitchen = SmartRoom::default("Kitchen".to_string());
        kitchen.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut house = SmartHouse::new("House".to_string());
        house.add_smart_room(kitchen).unwrap();
        assert!(!house.smart_rooms.is_empty())
    }

    #[test]
    fn delite_smart_room() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let mut kitchen = SmartRoom::default("Kitchen".to_string());
        kitchen.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut house = SmartHouse::new("House".to_string());
        house.add_smart_room(kitchen.clone()).unwrap();
        assert!(!house.smart_rooms.is_empty());
        house.remove_smart_room(kitchen).unwrap();
        assert!(house.smart_rooms.is_empty());
    }

    #[test]
    fn get_rooms() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let mut kitchen = SmartRoom::default("Kitchen".to_string());
        kitchen.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut hall = SmartRoom::default("Hall".to_string());
        hall.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut bathroom = SmartRoom::default("Bathroom".to_string());
        bathroom.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
        let mut living = SmartRoom::default("Living room".to_string());
        living.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        living.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();

        let mut house = SmartHouse::new("House".to_string());

        house.add_smart_room(kitchen).unwrap();
        house.add_smart_room(bathroom).unwrap();
        house.add_smart_room(living).unwrap();
        assert!(!house.get_rooms().is_empty())
    }

    #[test]

    fn create_report() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let socket_borrow = SmartSocket::default("Smart_socket".to_string());
        let mut kitchen = SmartRoom::default("Kitchen".to_string());
        kitchen.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut hall = SmartRoom::default("Hall".to_string());
        hall.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
        let mut bathroom = SmartRoom::default("Bathroom".to_string());
        bathroom.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
        let mut living = SmartRoom::default("Living room".to_string());
        living.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
        living.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();


        let mut house = SmartHouse::new("House".to_string());

        house.add_smart_room(kitchen).unwrap();
        house.add_smart_room(bathroom).unwrap();
        house.add_smart_room(living).unwrap();

        let info_provider_1 = OwningDeviceInfoProvider {socket};

        let info_provider_2 = BorrowingDeviceInfoProvider {
            socket: &socket_borrow,
            thermo: &thermo,
        };

        assert!(!house.create_report(info_provider_1).is_empty());
        assert!(!house.create_report(info_provider_2).is_empty());
    }
}
