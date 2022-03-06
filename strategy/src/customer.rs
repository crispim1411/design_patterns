use crate::ticket::SupportTicket;
use crate::ordering::TicketOrderingStrategy;

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