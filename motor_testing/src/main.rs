use mavio::default_dialect::enums::mav_frame;
use mavio::dialects::Common;
use mavio::prelude::*;
use mavio::default_dialect as dialect;
use dialect::enums::{MavAutopilot, MavModeFlag, MavState, MavType, MavCmd};
use embedded_io_adapters;

// fn main() -> std::io::Result<()> {
//     let serial = serialport::new("/dev/cu.usbmodem21101", 57600).open()?;

//     let reader = StdIoReader::new(serial);
//     let mut receiver = Receiver::versionless(reader);

//     for _i in 0..100 {
//         let frame = receiver.recv().unwrap();

//         // Validate MAVLink frame
//         if let Err(err) = frame.validate_checksum::<Ardupilotmega>() {
//             eprintln!("Invalid checksum: {err:?}");
//             continue;
//         }

//         // Decode and handle Heartbeat messages
//         if let Ok(Ardupilotmsega::Heartbeat(msg)) = frame.decode() {
//             println!(
//                 "HEARTBEAT #{}: mavlink_version={:#?}",
//                 frame.sequence(),
//                 msg.mavlink_version,
//             );
//         }
//     }

//     Ok(())
// }
// hi there
fn main() {
    // Create a TCP client sender
    let serial = serialport::new("/dev/cu.usbmodem21101", 57600).open().unwrap();

    let mut sender = Sender::new(StdIoWriter::new(serial));
    let mut receiver = Receiver::versionless(StdIoReader::new(serial));

    // Create an endpoint that represents a MAVLink device speaking `MAVLink 2` protocol
    let endpoint = Endpoint::v2(MavLinkId::new(15, 42));

    // let arm_message = dialect::messages::set_mode::SetMode {
    //     target_system: 1,
    //     base_mode: "arm",
    //     custom_mode: 
    // }

    // Create a message
    let motor_message = dialect::messages::command_long::CommandLong {
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
        confirmation: 0,
    };
    println!("MOTOR MESSAGE:", {motor_message:?});

    for i in 0..10 {
        // Receive the current MAVLink frame
        let frame = receiver.recv_frame();

        // Validate MAVLink frame
        if let Err(err) = frame.validate_checksum::<Minimal>() {
            eprintln!("Invalid checksum: {err:?}");
            continue;
        }
        match Ok(frame.decode().unwrap()) {
            Common::Attitude(msg) => {
                println!("IMU Yaw (rad): {}", msg.yaw);
                println!("IMU Pitch (rad): {}", msg.pitch);
                println!("IMU Roll (rad): {}", msg.roll);
            },
        }
        /* if let Ok(Common::Attitude(msg)) = frame.decode() {
            println!("IMU Yaw (rad): {}", msg.yaw);
        } */

        // Build the next frame for this endpoint.
        // All required fields will be populated, including frame sequence counter.
        let frame = endpoint.next_frame(&motor_message).unwrap();

        sender.send(&frame);
        println!("FRAME #{} sent: {:#?}", i, frame);
    }
}