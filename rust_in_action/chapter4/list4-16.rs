#![allow(unused_variables)]
#![allow(dead_code)]

/**/
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug)]
enum StatusMessage {
    Ok,
}

//
#[derive(Debug)]
struct GroundStation;

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

fn main() {
    println!("Hello Worrld");

    /************************************/
    let sat_a = CubeSat { id: 0 };
    let a_status = check_status(sat_a);
    // move sat_a
    // impl Copy Trait
    println!("a:{:?}", a_status);

    let a_status = check_status(sat_a);
    println!("a:{:?}", a_status);

    /************************************/

    let mut mailbox = Mailbox { messages: vec![] };
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    for sat_id in &sat_ids {
        let sat = base.connect(*sat_id);
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
}
