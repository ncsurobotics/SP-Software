use mavio::prelude::*;
use mavio::io::{StdIoReader, StdIoWriter};
use mavio::{Receiver, Sender, default_dialect as dialect};
use mavio::dialects::All;

use dialect::enums::MavCmd;
use dialect::messages::{CommandLong, ManualControl, Attitude};

use serialport::SerialPort;

use tokio::time::{sleep, Duration};

fn main() {
    // Create a serial client sender
    let serial = serialport::new("/dev/cu.usbmodem21101", 57600).open().unwrap();
    // Create an endpoint that represents a MAVLink device speaking `MAVLink 2` protocol
    let endpoint = Endpoint::v2(MavLinkId::new(15, 42));

    let i = 0;       
    loop {
        sender_periodic(serial.clone(), endpoint, i);
        receiver_periodic(serial.clone(), endpoint, i);
        i += 1;
    }
}

async fn sender_periodic(serial: Box<dyn SerialPort>, endpoint: Endpoint<V2>, index: usize) {
    let frames = [
        ("Motor Test",
            endpoint.next_frame(&CommandLong {
                target_system: 1,
                target_component: 1,
                command: MavCmd::DoMotorTest,
                param1: 1.0,
                param2: 0.0,
                param3: 10.0,
                param4: 0.0,
                param5: 1.0,
                param6: 0.0,
                param7: 0.0,
                confirmation: 0
            }).unwrap()
        ),
        ("Manual Control", 
            endpoint.next_frame(&ManualControl {
                target: 1,
                x: 0,
                y: 0,
                z: 0,
                r: 0,
                buttons: 0,
                buttons2: todo!(),
                enabled_extensions: todo!(),
                s: todo!(),
                t: todo!(),
                aux1: todo!(),
                aux2: todo!(),
                aux3: todo!(),
                aux4: todo!(),
                aux5: todo!(),
                aux6: todo!()
            }).unwrap()
        )
    ];

    let sender = Sender::new::<V2>(StdIoWriter::new(serial));
    for frame in frames.iter() {
        // Send the frames over the serial connection to the endpoint device
        sender.send(&frame.1).unwrap();
        println!("{} frame #{} sent: {:#?}", frame.0, index, frame.1);
        sleep(Duration::from_millis(1000)).await;
    }

}

async fn receiver_periodic(serial: Box<dyn SerialPort>, endpoint: Endpoint<V2>, index: usize) {
    // Create MAVLink sender and receiver over the serial connection
    let mut receiver = Receiver::versioned(StdIoReader::new(serial), V2);

    // Receive the current MAVLink frame
    let frame = receiver.recv().unwrap();
    let message_result = frame.decode_message();

    // Validate MAVLink frame
    if let Err(err) = frame.validate_checksum::<All>() {
        eprintln!("Invalid checksum: {err:?}");
        return;
    }
    if let Err(err) = message_result {
        eprintln!("Invalid message: {err:?}");
        return;
    }

    match frame.message_id() {
        Attitude::ID => {
            let message: Attitude = message_result.unwrap();
            println!("IMU Yaw (rad): {}", message.yaw);
            println!("IMU Pitch (rad): {}", message.pitch);
            println!("IMU Roll (rad): {}", message.roll);
        }
        _ => {}
    }
}