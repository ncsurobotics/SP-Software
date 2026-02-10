use sw8s_rust_lib::comms::control_board::ControlBoard;
use tokio::io::WriteHalf;

#[tokio::main]
async fn main() {
    let board = ControlBoard::serial("/dev/cu.usbmodem2101").await.expect("Connection Failed");
    
}