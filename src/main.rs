mod args;
use args::Args;

fn main() {
    let args:Args = Args::new();
    println!("{:?}",args);
}
