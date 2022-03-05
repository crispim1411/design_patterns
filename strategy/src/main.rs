use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::Alphanumeric;

trait TicketOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket>;
}

struct FIFOOrderingStrategy;

impl TicketOrderingStrategy for FIFOOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        return list.clone();
    }
}

struct FILOOrderingStrategy;

impl TicketOrderingStrategy for FILOOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        list.clone().reverse();
        return list.to_owned();
    }
}

struct RandomOrderingStrategy;

impl TicketOrderingStrategy for RandomOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        let mut copy_list = list.clone();
        let mut rng = rand::thread_rng();
        copy_list.shuffle(&mut rng);
        return copy_list
    }
}

struct BlackHoleOrderingStrategy;

impl TicketOrderingStrategy for BlackHoleOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        return vec![];
    }
}

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
    tickets: Vec<SupportTicket>
}

impl CustomerSupport {
    fn new() -> CustomerSupport {
        CustomerSupport { tickets: vec![] }
    }
}

impl CustomerSupport {
    fn create_ticket(&mut self, customer: String, issue: String) {
        self.tickets.push(SupportTicket::new(customer, issue))
    }

    fn process_tickets(&self, processing_strategy: Box<dyn TicketOrderingStrategy>) {
        let ticked_list = processing_strategy.create_ordering(&self.tickets);

        if ticked_list.len() == 0 {
            println!("There are no tickets to process. Well done!");
            return;
        }

        for ticket in ticked_list {
            self.process_ticket(&ticket);
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
    let mut app = CustomerSupport::new();
    
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
    app.process_tickets(Box::new(BlackHoleOrderingStrategy));
}
