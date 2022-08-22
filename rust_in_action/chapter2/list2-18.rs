fn main() {
    let search_term = "picture";
    let quote = "\
Every face ,every shop,bed room window, public-house , and
dark square is a picture feverishlyturned--in search of what?
It is the same with books.
What dow wee seek through millions of oages?";
    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
