fn main() {
    let _needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 1430, 4862];
    for item in &haystack {
        /**/
        let result = match item {
            42 | 132 => "hit",
            _ => "miss",
        };
        println!("{}:{}", item, result);
    }

    /* */
    {
        let a = 10;
        let b = 20;
        let c = 30;
        let array = [&a, &b, &c];
        for item in &array {
            let result = match item {
                20 => "hit",
                _ => "miss",
            };
            println!("{}:{}", item, result);
        }
    }
}
