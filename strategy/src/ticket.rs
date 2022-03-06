use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

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

fn generate_id(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}