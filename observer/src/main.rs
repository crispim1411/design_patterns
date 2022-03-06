use std::rc::Rc;
use observer::{User, Event};
use observer::{Observer, Subject};

fn main() {
    // creating users
    let (user1, user2, user3, user4) = 
    (
        User {
            name: String::from("Jorginaldo (Engenharia)"),
            email: String::from("jorjis@email.com"),
        },
        User {
            name: String::from("Astrogilda (História)"),
            email: String::from("astro@email.com"),
        },
        User {
            name: String::from("Margorete (Geologia)"),
            email: String::from("margo@email.com"),
        },
        User {
            name: String::from("Rita (Medicina)"),
            email: String::from("ritalee@email.com"),
        }
    );
    // references
    let rc_user1: Rc<dyn Observer> = Rc::new(user1);
    let rc_user2: Rc<dyn Observer> = Rc::new(user2);
    let rc_user3: Rc<dyn Observer> = Rc::new(user3);
    let rc_user4: Rc<dyn Observer> = Rc::new(user4);
    // events
    let mut quim_event = Event::new(String::from("Química"));
    let mut bio_event = Event::new(String::from("Biologia"));
    let mut mat_event = Event::new(String::from("Matemática"));
    let mut fis_event = Event::new(String::from("Física"));
    let mut geo_event = Event::new(String::from("Geografia"));
    let mut hist_event = Event::new(String::from("História"));
    let mut lib_event = Event::new(String::from("Libras"));
    
    //subscribing
    //estudante 1 inscrito em Química, Física, Matemática e Libras
    quim_event.register(&rc_user1);
    fis_event.register(&rc_user1);
    mat_event.register(&rc_user1);
    lib_event.register(&rc_user1);

    //estudante 2 inscrito em Geografia, História e Libras
    geo_event.register(&rc_user2);
    hist_event.register(&rc_user2);
    lib_event.register(&rc_user2);

    //estudante 3 inscrito em Matemática, Geografia e Libras
    mat_event.register(&rc_user3);
    geo_event.register(&rc_user3);
    lib_event.register(&rc_user3);

    //estudante 4 inscrito em Biologia, Química e Libras
    bio_event.register(&rc_user4);
    quim_event.register(&rc_user4);
    lib_event.register(&rc_user4);

    println!("Strong references");
    println!("user1: {}", Rc::strong_count(&rc_user1));
    println!("user2: {}", Rc::strong_count(&rc_user2));
    println!("user3: {}", Rc::strong_count(&rc_user3));
    println!("user4: {}\n", Rc::strong_count(&rc_user4));

    // post data
    quim_event.post_event(String::from("Início aula química"));
    lib_event.post_event(String::from("Aula de Libras cancelada por falta de professor"));
    geo_event.post_event(String::from("Aula de geografia cancelada"));
    drop(lib_event);
    drop(geo_event);  

    // remove user
    println!("-------------------------------------------");
    println!("Removendo estudantes da aula de matemática");
    println!("-------------------------------------------\n");
    mat_event.unregister(&rc_user1);
    mat_event.unregister(&rc_user3);

    println!("Strong references");
    println!("user1: {}", Rc::strong_count(&rc_user1));
    println!("user2: {}", Rc::strong_count(&rc_user2));
    println!("user3: {}", Rc::strong_count(&rc_user3));
    println!("user4: {}\n", Rc::strong_count(&rc_user4));

    //post data
    mat_event.post_event(String::from("Prova de matemática semana que vem"));
    bio_event.post_event(String::from("Inicio aula de biologia"));
    fis_event.post_event(String::from("Trabalho de física no email"));
    hist_event.post_event(String::from("Trabalho de história em grupo"));
    quim_event.post_event(String::from("Próxima aula de química no laboratório"));
}
