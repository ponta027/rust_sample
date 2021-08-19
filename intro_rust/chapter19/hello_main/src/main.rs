#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}
fn main() {
    println!("Hello, world!");
    let mut a = Person {
        id: 100,
        name: "test".to_string(),
        age: 40,
    };
    let b = Person {
        id: 100,
        name: "test2".to_string(),
        age: 40,
    };
    let c = Person {
        id: 200,
        name: "test3".to_string(),
        age: 40,
    };
    assert_eq!("test", a.name);
    assert_eq!("test2", b.name);
    assert_eq!("test3", c.name);
    println!("a is {:?}", a);
    println!("c is {:?}", c);
    //    assert_eq!(a,b);
    a.age += 1;
    println!("a is {:?}", a);
    assert_ne!(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1, 2));
    }
}

fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    if ans < 0 {
        0
    } else {
        ans
    }
}
