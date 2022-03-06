use rand::prelude::SliceRandom;
use crate::SupportTicket;

pub trait TicketOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket>;
}

pub struct FIFOOrderingStrategy;
impl TicketOrderingStrategy for FIFOOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        return list.clone();
    }
}

pub struct FILOOrderingStrategy;
impl TicketOrderingStrategy for FILOOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        let mut copy_list = list.clone();
        copy_list.reverse();
        return copy_list;
    }
}

pub struct RandomOrderingStrategy;
impl TicketOrderingStrategy for RandomOrderingStrategy {
    fn create_ordering(&self, list: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        let mut copy_list = list.clone();
        let mut rng = rand::thread_rng();
        copy_list.shuffle(&mut rng);
        return copy_list
    }
}

pub struct BlackHoleOrderingStrategy;
impl TicketOrderingStrategy for BlackHoleOrderingStrategy {
    fn create_ordering(&self, _: &Vec<SupportTicket>) -> Vec<SupportTicket> {
        return vec![];
    }
}