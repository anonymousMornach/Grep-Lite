extern crate regex;
extern crate clap;
use regex::Regex;
use clap::{App, Arg};

fn main() {
    let args = App::new("grep-lite")
    .version("1.0.0");


    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?
but i love the way the world works";
    let context_lines = 1;
    pattern(&search_term, &quote);
    manual(&search_term, &quote);
    enumerate(&search_term, &quote);
    advanced(&search_term, &quote, context_lines );
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

//enumerate method
fn enumerate(search_term:&str, quote:&str)
{
    for (idx, line) in quote.lines().enumerate()
    {
        if line.contains(search_term)
        {
            let line_num = idx + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn advanced(search_term:&str, quote:&str, context_lines:usize)
{
    let re = Regex::new(search_term).unwrap();
    //Initialization
    let mut tags :Vec<usize> = Vec::new();
    let mut ctx :Vec<Vec<(usize, String)>> = Vec::new();
    for(i, line) in quote.lines().enumerate(){
        match re.find(line)
        {
            Some(_) =>{
                tags.push(i);
                let v: Vec<(usize, String)> = Vec::with_capacity(2*context_lines + 1);
                ctx.push(v)
            },
            None => ()
        }
    }

    if tags.len() == 0{
        return;
    }

    //Pass 2

    for (i, line ) in quote.lines().enumerate()
    {
        for(j, tag) in tags.iter().enumerate()
        {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;

            if(i >= lower_bound) && (i <= upper_bound)
            {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    //Output
    for local_ctx in ctx.iter()
    {
        for &(i, ref line) in local_ctx.iter(){
            let line_num = i + 1;
            println!("{}: {}", line_num, line)
        }
    }
}