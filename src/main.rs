use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short, long)]
    operation: String,
    a: f64,
    b: f64,
}

fn main() {
    let args = Cli::from_args();
    match args.operation.as_str() {
        "add" => println!("Result: {}", args.a.add(args.b)),
        "sub" => println!("Result: {}", args.a.sub(args.b)),
        "mul" => println!("Result: {}", args.a.mul(args.b)),
        "div" => {
            if args.b != 0.0 {
                println!("Result: {}", args.a.div(args.b));
            } else {
                println!("Error: Division by zero");
            }
        }
        _ => println!("Error: Unsupported operation"),
    }
}