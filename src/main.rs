use std::time::Duration;

use sw9s_lib::comms::control_board::ControlBoard;
use tokio::io::WriteHalf;
use tokio::time::sleep;


#[tokio::main]
async fn main() {
    let board = ControlBoard::serial("/dev/serial/by-id/usb-STMicroelectronics_Control_Board_v2__Virtual_COM_Port__3631363230325101002A001F-if00").await.expect("Connection Failed");
    let _ = board.raw_speed_set([0.2, 0.2, 0.3, 0.2, 0.1, 0.0, -0.1, -0.2]).await;
    sleep(Duration::from_millis(2000)).await;
    let _ = board.raw_speed_set([-0.2, -0.2, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2]).await;
    sleep(Duration::from_millis(2000)).await;
    let _ = board.raw_speed_set([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).await;
}
