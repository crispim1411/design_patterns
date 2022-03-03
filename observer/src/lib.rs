use std::rc::{Rc, Weak};

pub trait Observer {
    fn notify(&self, event: &String);
}

pub trait Subject {
    fn register(&mut self, observer: Weak<dyn Observer>);
    fn unregister(&mut self, observer: &Rc<dyn Observer>);
    fn post_event(&self, data: String);
}