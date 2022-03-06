pub mod ordering;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::ordering::TicketOrderingStrategy;

#[derive(Clone)]
pub struct SupportTicket {
    pub id: String,
    pub customer: String,
    pub issue: String,
}

impl SupportTicket {
    pub fn new(customer: String, issue: String) -> SupportTicket {
        SupportTicket {
            id: generate_id(8),
            customer,
            issue
        }
    }
}

pub struct CustomerSupport {
    tickets: Vec<SupportTicket>
}

impl CustomerSupport {
    pub fn new() -> CustomerSupport {
        CustomerSupport { tickets: vec![] }
    }
}

impl CustomerSupport {
    pub fn create_ticket(&mut self, customer: String, issue: String) {
        self.tickets.push(SupportTicket::new(customer, issue))
    }

    pub fn process_tickets(&self, processing_strategy: impl TicketOrderingStrategy) {
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