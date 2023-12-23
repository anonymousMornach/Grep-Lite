fn main() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";
    pattern(&search_term, &quote);
    manual(&search_term, &quote);
}
//using pattern
fn pattern(search_term:&str, quote:&str)
{
    for line in quote.lines()
{
    if line.contains(search_term)
    {
        println!("{}", line );
    }
}

}


//manual method
fn manual(search_term:&str, quote:&str)
{
    let mut line_num:usize = 1;
    for line in quote.lines()
    {

        if line.contains(search_term)
        {
            println!("{}: {}", line_num, line);
        }
        line_num += 1;
    }
}