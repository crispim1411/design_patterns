use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::Alphanumeric;

// trait Strategy {
//     type T;
//     fn algorithm(data: Self::T) -> Self::T;
// }

#[derive(Clone)]
struct SupportTicket {
    id: String,
    customer: String,
    issue: String,
}

impl SupportTicket {
    fn new(customer: String, issue: String) -> SupportTicket {
        SupportTicket {
            id: generate_id(8),
            customer,
            issue
        }
    }
}

struct CustomerSupport {
    tickets: Vec<SupportTicket>,
    processing_strategy: &'static str,
}

impl CustomerSupport {
    fn new(processing_strategy: &'static str) -> CustomerSupport {
        CustomerSupport {
            tickets: vec![],
            processing_strategy
        }
    }
}

impl CustomerSupport {
    fn create_ticket(&mut self, customer: String, issue: String) {
        self.tickets.push(SupportTicket::new(customer, issue))
    }

    fn process_tickets(&self) {
        if self.tickets.len() == 0 {
            println!("There are no tickets to process. Well done!");
            return;
        }
        
        match self.processing_strategy {
            "fifo" => {
                for ticket in &self.tickets {
                    self.process_ticket(&ticket)
                }
            },
            "filo" => {
                for ticket in self.tickets.iter().rev() {
                    self.process_ticket(&ticket)
                }
            },
            "random" => {
                let mut rng = rand::thread_rng();
                let mut copy_list = self.tickets.clone();
                copy_list.shuffle(&mut rng);
                for ticket in copy_list {
                    self.process_ticket(&ticket)
                }
            },
            _ => println!("That ordenation isn't listed on system.")
        }
    }

    fn process_ticket(&self, ticket: &SupportTicket) {
        println!("==================================");
        println!("Processing ticket id: {}", ticket.id);
        println!("Customer: {}", ticket.customer);
        println!("Issue: {}", ticket.issue);
        println!("==================================");
    }
}

fn generate_id(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn main() {
    // create the application
    let mut app = CustomerSupport::new("random");
    
    // register a few tickets
    app.create_ticket(
        String::from("1. John Smith"), 
        String::from("My computer makes strange sounds!")
    );
    app.create_ticket(
        String::from("2. Linus Sebastian"), 
        String::from("I can't upload any videos, please help.")
    );
    app.create_ticket(
        String::from("3. Arjan Egges"), 
        String::from("VSCode doesn't automatically solve my bugs.")
    );
    app.create_ticket(
        String::from("4. Charles Delton"), 
        String::from("My network connection is unstable.")
    );

    // process the tickets
    app.process_tickets()
}
