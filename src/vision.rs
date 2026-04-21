use opencv::core;
use opencv::core::Mat;
use opencv::videoio::{self, VideoCaptureTrait};
use std::time::Duration;
use sw9s_lib::comms::control_board::{self, ControlBoard};
use tokio::io::WriteHalf;
use tokio::time::sleep;
use tokio_serial::SerialStream;
pub struct VisionControl<'a>{
    video_capture: videoio::VideoCapture,
    board: &'a ControlBoard<WriteHalf<SerialStream>>,
}

impl<'a> VisionControl<'a> {
    
    pub fn new(video_capture: videoio::VideoCapture, board: &'a ControlBoard<WriteHalf<SerialStream>>,) -> Self {
        Self {video_capture, board} // Returns the struct instance
    }

    pub fn get_frame(&mut self) -> core::Mat{
        let mut frame = Mat::default();
        let _ = self.video_capture.read(&mut frame);
        return frame;
    }
}