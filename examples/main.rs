use smarthouse::devices::*;
use smarthouse::smarthouse::*;
use smarthouse::smartroom::*;

fn main() {
    // Инициализация устройств
    let socket = SmartSocket::default("Smart_socket".to_string());
    let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
    let socket_borrow = SmartSocket::default("Smart_socket".to_string());
    
    //Инициализация комнат

    let mut kitchen = SmartRoom::default("Kitchen".to_string());
    kitchen.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
    let mut hall = SmartRoom::default("Hall".to_string());
    hall.add_smart_device(Device::SmartSocket(socket.clone())).unwrap();
    let mut bathroom = SmartRoom::default("Bathroom".to_string());
    bathroom.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
    let mut living = SmartRoom::default("Living room".to_string());
    living.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
    living.add_smart_device(Device::SmartThermometr(thermo.clone())).unwrap();
    // Инициализация дома

    let mut house = SmartHouse::new("House".to_string());
    house.add_smart_room(kitchen).unwrap();
    house.add_smart_room(bathroom).unwrap();
    house.add_smart_room(living).unwrap();
    house.add_smart_room(hall.clone()).unwrap();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {socket};

    let report1 = house.create_report(info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    house.remove_smart_room(hall).unwrap();
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket_borrow,
        thermo: &thermo,
    };

    let report2 = house.create_report(info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
