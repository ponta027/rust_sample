#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 5);
    }

    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(2, 1));
        assert_eq!(10, add(0, 10));
        assert_eq!(99, add(10, 0));
        assert_eq!(20, add(10, 10));
    }
    #[test]
    fn test_add_zero() {
        assert_eq!(0, add(0, 0));
    }
    #[test]
    fn test_add_under_zero() {
        assert_eq!(0, add(-1, -1));
    }
    #[test]
    fn test_add_double() {
        assert_eq!(1.2, add_double(1.0, 0.2));
        assert_eq!(0.0, add_double(0.0, 0.0));
        assert_eq!(0.0, add_double(-1.0, -1.0));
    }

    #[test]
    fn test_add_str() {
        assert_eq!("test hoge", add_str("test", "hoge"));
        assert_eq!("", add_str("", ""));
    }
    // 19.2
    #[test]
    fn test_equal() {
        assert_eq!(2, 1 + 1);
        assert_eq!(1.123, 1.0 + 0.123);
        assert_eq!(true, 1.0 == 1.0);
        assert_eq!("rust", " rust ".trim());
    }
    #[test]
    fn test_not_equal() {
        assert_ne!(0, 1 + 1);
        assert_ne!("javascript", "Java");
    }

    #[test]
    fn test_equal_instance() {
        let mut a = Person {
            id: 100,
            name: "test".to_string(),
            age: 40,
        };
        let b = Person {
            id: 100,
            name: "test".to_string(),
            age: 40,
        };
        let c = Person {
            id: 200,
            name: "test".to_string(),
            age: 40,
        };

        assert_eq!( a , a );
        assert_eq!( a , b );
        assert_eq!( a , c );
        a.age += 1;
        let x = &a ;
        assert_eq!( a , *x );
    }
    #[test]
    fn test_asser(){
        assert!(true);
        assert!(1==1);
    }
}

fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    ans
}

fn add_double(x: f32, y: f32) -> f32 {
    let ans = x + y;
    if ans < 0.0 {
        0.0
    } else {
        ans
    }
}

fn add_str(x: &str, y: &str) -> String {
    let ans = format!("{} {}", x, y);
    ans.trim().to_string()
}
