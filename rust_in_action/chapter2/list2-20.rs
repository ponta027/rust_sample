fn main() {
    let search_term = "picture";
    let quote = "\
Every face ,every shop,bed room window, public-house , and
dark square is a picture feverishlyturned--in search of what?
It is the same with books.
What dow wee seek through millions of oages?";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}:{}", line, line_num);
        }
    }
}
