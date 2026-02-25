use sw9s_lib::comms::control_board::ControlBoard;
use tokio::io::WriteHalf;

#[tokio::main]
async fn main() {
    let board = ControlBoard::serial("/dev/serial/by-id/usb-STMicroelectronics_Control_Board_v2__Virtual_COM_Port__3631363230325101002A001F-if00").await.expect("Connection Failed");
    board
        .stability_2_speed_set(0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .await;
}
