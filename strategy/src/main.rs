use strategy::CustomerSupport;
use strategy::ordering::*;

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
    app.process_tickets(RandomOrderingStrategy);
}
