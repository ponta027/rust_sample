mod method;
mod result;
mod result2;
mod sample;
use sample::sample_lib;

        
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn print_person(pa: &Person) {
    println!(
        "ID={},name={},age:{},addr:{}",
        pa.id, pa.name, pa.age, pa.addr
    );
}

fn add_age(pa: &mut Person) {
    pa.age += 1;
}

fn new_person(id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}

/* */

struct A {
    id: i32,
}
struct B {
    id: i32,
    id2: i32,
}
struct C {
    id: i32,
    name: String,
}
struct D {
    id: i32,
    name: &'static str,
}
struct E {
    id: i32,
    v: Vec<u8>,
}
struct F {
    id: i32,
    v: [u8; 100],
}

fn get_struct_size() -> i32 {
    let a = A { id: 10 };
    println!("A size of is {}", std::mem::size_of::<A>());
    let b = B { id: 10, id2: 20 };
    println!("B size of is {}", std::mem::size_of::<B>());
    println!("{},{}", b.id, b.id2);

    let c = C {
        id: 10,
        name: String::from(""),
    };
    println!("C size of is {}", std::mem::size_of::<C>());
    println!("{},{}", c.id, c.name);

    let d = D { id: 10, name: "" };
    println!("D size of is {}", std::mem::size_of::<D>());
    println!("{},{}", d.id, d.name);

    let e = E { id: 10, v: vec![1] };
    println!("E size of is {}", std::mem::size_of::<E>());
    println!("{},{}", e.id, e.v[0]);

    let f = F { id: 10, v: [1;100] };
    println!("F size of is {}", std::mem::size_of::<F>());
    println!("{},{}", f.id, f.v[0]);

    a.id
}

fn main() {
    let pa = Person {
        id: 100,
        name: String::from("test"),
        age: 40,
        addr: String::from("Tokyo"),
    };
    println!("Hello, world!");
    println!(
        "ID={},name={},age:{},addr:{}",
        pa.id, pa.name, pa.age, pa.addr
    );

    //
    let mut pa = Person {
        id: 100,
        name: String::from("test"),
        age: 40,
        addr: String::from("Tokyo"),
    };
    pa.age += 1;
    println!(
        "ID={},name={},age:{},addr:{}",
        pa.id, pa.name, pa.age, pa.addr
    );

    add_age(&mut pa);
    print_person(&pa);
    let pa2 = new_person(200, "hoge");
    print_person(&pa2);

    //
    let mut people = vec![pa, pa2];
    people.push(new_person(10, "aaa"));
    people.push(new_person(20, "bbb"));
    for p in &people {
        print_person(p);
    }

    println!("no field stuct\n");
    /************************************/
    struct Color(f32, f32, f32);
    let yellow = Color(0.1, 0.1, 0.1);
    println!("R:{:.2} , G{:2} , B{:2}", yellow.0, yellow.1, yellow.2);

    let ret = get_struct_size();
    println!("ret={}", ret);

    /* */
    method::do_sample();
    /* */
    result::do_result();
    result2::sample();
    /* */
    sample::example();
    sample_lib::hoge();
}
