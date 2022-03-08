use crate::utils;
use crate::devices::Buffer;

pub trait StreamingService {
    fn start_stream(&mut self) -> String;
    fn fill_buffer(&mut self, stream_reference: &str);
    fn stop_stream(&mut self, stream_reference: &str);
    fn add_device(&mut self, device: Box<dyn Buffer>);
    fn retrieve_buffer_data(&self) -> Vec<String>;
}

pub struct YoutubeStreamingService {
    devices: Vec<Box<dyn Buffer>>,
    stream_reference: String, 
    buffer_data: Vec<String>,
}

impl YoutubeStreamingService {
    pub fn new() -> Self {
        YoutubeStreamingService {
            devices: vec![],
            stream_reference: String::new(), 
            buffer_data: vec![],
        }
    }
}

impl StreamingService for YoutubeStreamingService {
    fn start_stream(&mut self) -> String {
        self.stream_reference = utils::generate_id(8);
        println!("Starting Youtube stream with reference: {}", self.stream_reference);
        return self.stream_reference.clone();
    }

    fn fill_buffer(&mut self, stream_reference: &str) {
        self.buffer_data = self.retrieve_buffer_data();
        println!("Received buffer data: {:?}. Sending to Youtube stream: {}", self.buffer_data, stream_reference);
    }

    fn stop_stream(&mut self, stream_reference: &str) {
        println!("Closing Youtube stream with reference: {}", stream_reference);
    }

    fn add_device(&mut self, device: Box<dyn Buffer>) {
        self.devices.push(device); 
    }

    fn retrieve_buffer_data(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.retrieve_data()).collect()
    }
}

pub struct TwitchStreamingService {
    devices: Vec<Box<dyn Buffer>>,
    stream_reference: String, 
    buffer_data: Vec<String>,
}

impl TwitchStreamingService {
    pub fn new() -> Self {
        TwitchStreamingService {
            devices: vec![],
            stream_reference: String::new(), 
            buffer_data: vec![],
        }
    }
}

impl StreamingService for TwitchStreamingService {
    fn start_stream(&mut self) -> String {
        self.stream_reference = utils::generate_id(8);
        println!("Starting Twitch stream with reference: {}", self.stream_reference);
        return self.stream_reference.clone();
    }

    fn fill_buffer(&mut self, stream_reference: &str) {
        self.buffer_data = self.retrieve_buffer_data();
        println!("Received buffer data: {:?}. Sending to Twitch stream: {}", self.buffer_data, stream_reference);
    }

    fn stop_stream(&mut self, stream_reference: &str) {
        println!("Closing Twitch stream with reference: {}", stream_reference);
    }

    fn add_device(&mut self, device: Box<dyn Buffer>) {
        self.devices.push(device);
    }

    fn retrieve_buffer_data(&self) -> Vec<String> {
        self.devices.iter().map(|d| d.retrieve_data()).collect()
    }
}