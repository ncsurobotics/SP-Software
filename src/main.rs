use std::time::Duration;

use sw9s_lib::comms::control_board::{self, ControlBoard};
use tokio::io::WriteHalf;
use tokio::time::sleep;
use tokio_serial::SerialStream;


#[tokio::main]

async fn main() {


    let board = ControlBoard::serial("/dev/serial/by-id/usb-STMicroelectronics_Control_Board_v2__Virtual_COM_Port__3631363230325101002A001F-if00").await.expect("Connection Failed");

    //This arm variable needs to arm and disarm the system. It should be connected to a physical switch. 
    let arm_bool: bool = false;
    tokio::spawn(async move {
        while !arm_bool{
            sleep(Duration::from_secs(1)).await;
        }
        dbg!("I AM ARMED");
    });
    let motor_matrix: [[f32; 6];6] = [
        [1.0, 0.0, -1.0, 1.0, 1.0, 1.0],
        [-1.0, 0.0, -1.0, 1.0, -1.0, -1.0],
        [-1.0, 0.0, -1.0, -1.0, 1.0, 1.0],
        [1.0, 0.0, -1.0, -1.0, -1.0, -1.0],
        [0.0, 1.0, -1.0, 1.0, 1.0, 1.0],
        [0.0, 1.0, -1.0, 1.0, 1.0, -1.0]
    ];
    set_motor_matrix(board, motor_matrix);


    //I don't really know how we're supposed to edit arm and also there is no meb on the raspberry pi

}

    fn set_motor_matrix(board: ControlBoard<WriteHalf<SerialStream>>,matrix: [[f32; 6];6]){
        let mut count: u8 = 0;
        for row in matrix{
            let _ = board.motor_matrix_set(count, row[0], row[1], row[2], row[3], row[4], row[5]);
            count += 1;
        }
    }
    

//i guess we have 6 motors
