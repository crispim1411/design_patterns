pub trait Buffer {
    fn retrieve_data(&self) -> String;
}

pub struct Webcam;
impl Buffer for Webcam {
    fn retrieve_data(&self) -> String {
        String::from("###WEBCAMDATA###")
    }
}

pub struct DSLRCamera;
impl Buffer for DSLRCamera {
    fn retrieve_data(&self) -> String {
        String::from("###DSLRDATA###")
    }
}
