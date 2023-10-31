use smarthouse::devices::*;
use smarthouse::smarthouse::*;
use smarthouse::smartroom::*;

fn main() {
    // Инициализация устройств
    let socket = SmartSocket::default("Smart_socket".to_string());
    let socket_borrow = SmartSocket::default("Smart_socket".to_string());
    let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
    //Инициализация комнат
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
    // Инициализация дома
    let mut house = SmartHouse::new("House".to_string());
    house.smart_rooms.insert("Kitchen".to_string(), kitchen);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket };

    let report1 = house.create_report(info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket_borrow,
        thermo: &thermo,
    };

    let report2 = house.create_report(info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
