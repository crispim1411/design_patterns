use bridge::services::*;
use bridge::devices::*;

fn main() {
    // create a device and a streaming service
    let mut service = YoutubeStreamingService::new();
    service.add_device(Box::new(Webcam));

    // start streaming
    let reference = service.start_stream();
    service.fill_buffer(&reference);
    service.stop_stream(&reference);

    // // create another device and streaming service
    let mut service2 = TwitchStreamingService::new();
    service2.add_device(Box::new(DSLRCamera));
    service2.add_device(Box::new(Webcam));

    // // start streaming there as well
    let reference2 = service2.start_stream();
    service2.fill_buffer(&reference2);
    service2.stop_stream(&reference2);
}




