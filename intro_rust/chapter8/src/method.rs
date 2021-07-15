pub fn do_sample() {
    println!("[START] method==============");
    argument();
    println!("new method");
    new_method();
    println!("[END] method==============");
}


struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID :i32 = 0;

impl Person {
    fn print_person(&self) {
        println!(
            "ID={},name={},age:{},addr:{}",
            self.id, self.name, self.age, self.addr
        );
    }
    fn print_t(&self, private: bool) {
        if private == true {
            println!("{}:{}", self.id, self.name);
        } else {
            println!(
                "ID={},name={},age:{},addr:{}",
                self.id, self.name, self.age, self.addr
            );
        }
    }

    fn to_str( &self ) -> String{
        let s = format!("{}:{}:({}) in {}",
        self.id, self.name , self.age , self.addr );
        s
    }

    fn add_age( &mut self , n:i32){
        self.age +=n
    }

    fn new(name :&str, age:i32, addr:&str ) -> Person{
        let id =unsafe{
                PERSON_ID +=1;
                PERSON_ID
        };
        Person{ 
            id: id,
            name : name.to_string(),
            age : age,
            addr : addr.to_string(),
        }

    }
}
fn argument() {
    println!("not argument");
    let pa = Person {
        id: 1,
        name: String::from("test"),
        age: 50,
        addr: String::from("Tokyo"),
    };

    pa.print_person();
    println!("has argument");
    pa.print_t(true);

    println!("return ");
    
    println!("{}",pa.to_str() );

    println!("mutable variable");
    let mut pa = Person {
        id: 1,
        name: String::from("test"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.add_age(10);
    pa.print_person();
}

fn new_method(){
    let mut people = Vec::<Person>::new();
    people.push( Person::new( "a",1,"Tokyo"));
    people.push( Person::new( "b",2,"Tokyo"));
    people.push( Person::new( "c",3,"Tokyo"));
    for p in &people{
        p.print_person();
    }
}
