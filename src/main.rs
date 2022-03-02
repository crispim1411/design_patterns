use std::rc::Rc;
use std::rc::Weak;
use observer::{Observer, Subject};

pub struct User {
    pub name: String,
    pub email: String
}

impl Observer for User {
    fn notify(&self, event: &String) {
        println!("{} recebeu: {}", self.name, event);
    }
}

pub struct Event {
    pub subject: String,
    pub subscribers: Vec<Weak<dyn Observer>>
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
        self.subscribers.push(Rc::downgrade(observer));
    }

    fn unregister(&mut self, observer: Rc<dyn Observer>) {
        drop(observer)
    }

    fn post_event(&self, data: String) {
        for subscriber in &self.subscribers {
            if let Some(subscriber) = subscriber.upgrade() {
                subscriber.notify(&data);
            }
        }
    }
}

fn main() {
    // creation
    let (user1, user2, user3, user4) = 
    (
        User {
            name: String::from("Jorginaldo"),
            email: String::from("jorjis@email.com"),
        },
        User {
            name: String::from("Astrogilda"),
            email: String::from("astro@email.com"),
        },
        User {
            name: String::from("Margorete"),
            email: String::from("margo@email.com"),
        },
        User {
            name: String::from("Rita"),
            email: String::from("ritalee@email.com"),
        }
    );
    
    let rc_user1: Rc<dyn Observer> = Rc::new(user1);
    let rc_user2: Rc<dyn Observer> = Rc::new(user2);
    let rc_user3: Rc<dyn Observer> = Rc::new(user3);
    let rc_user4: Rc<dyn Observer> = Rc::new(user4);

    let mut event = Event::new(String::from("Aula de química"));

    //subscribing
    event.register(&rc_user1);
    event.register(&rc_user2);
    event.register(&rc_user3);
    event.register(&rc_user4);

    // post data
    event.post_event(String::from("Intervalo"));

    // remove user
    event.unregister(rc_user2);
    event.unregister(rc_user4);

    //post data
    event.post_event(String::from("Fim da aula"));
}
