#![allow(unused_variables)]
#![allow(dead_code)]

//shared pointer
use std::cell::RefCell;
use std::rc::Rc;

/**/
#[derive(Debug)]
struct CubeSat {
    id: u64,
}

/**/
#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

/* */
#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

//
#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

/* */
impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(
        &self,                 //
        mailbox: &mut Mailbox, //
        msg: Message,
    ) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    sat_id
}
impl Clone for CubeSat {
    fn clone(&self) -> Self {
        println!("Clone CubeSat");
        CubeSat { id: self.id }
    }
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        println!("Clone StatusMessage");
        *self
    }
}

#[derive(Debug, Copy)]
enum StatusMessage {
    Ok,
}

fn main() {
    println!("Hello Worrld");

    let base = Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);
    {
        let mut base2 = base.borrow_mut();
        base2.radio_freq -= 12.34;
        println!("base2: {:?}", base2);
    }
    println!("base: {:?}", base);

    let mut base_3 = base.borrow_mut();

    base_3.radio_freq += 43.21;
    println!("base: {:?}", base);
    println!("base3: {:?}", base_3);

    /*
    let mut mailbox = Mailbox { messages: vec![] };
    let sat_ids = fetch_sat_ids();

    for sat_id in &sat_ids {
        let sat = base.connect(*sat_id);
        base.radio_freq = 12.34;
        base.send(
            &mut mailbox,
            Message {
                to: *sat_id,
                content: String::from("Hello!"),
            },
        );
        //
        println!("t0:{:?}", sat);
    }
    for sat_id in &sat_ids {
        let sat = base.connect(*sat_id);
        let msg = sat.recv(&mut mailbox);
        println!("t0:{:?} {:?}", sat, msg);
    }
    */
}
