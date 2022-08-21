fn main() {
    let _needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 1430, 4862];
    for item in &haystack {
        if _needle == *item {
            println!("{}", item);
        }
    }
}
