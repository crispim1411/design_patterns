use std::rc::Rc;

pub trait Observer {
    fn notify(&self, event: &String);
}

pub trait Subject {
    fn register(&mut self, observer: &Rc<dyn Observer>);
    fn unregister(&mut self, observer: &Rc<dyn Observer>);
    fn post_event(&self, data: String);
}

pub struct User {
    pub name: String,
    pub email: String
}

impl Observer for User {
    fn notify(&self, event: &String) {
        println!("{} recebeu: {}\n", self.name, event);
    }
}

pub struct Event {
    subject: String,
    subscribers: Vec<Rc<dyn Observer>>
}

impl Event {
    pub fn new(subject: String) -> Event {
        Event {
            subject,
            subscribers: vec![]
        }
    }
}

impl Subject for Event {
    fn register(&mut self, observer: &Rc<dyn Observer>) {
        let rc_ref = Rc::clone(&observer);
        self.subscribers.push(rc_ref);
    }

    fn unregister(&mut self, observer: &Rc<dyn Observer>) {
        if let Some(index) = self.subscribers.iter()
            .position(|item| Rc::ptr_eq(&observer, &item)) {
                self.subscribers.remove(index);
            }
    }

    fn post_event(&self, data: String) {
        match self.subscribers.len() {
            0 => println!("Não há estudantes cadastrados em {}\n", self.subject),
            _ => {
                for subscriber in &self.subscribers {
                    subscriber.notify(&data);
                }
            }
        }
    }
}