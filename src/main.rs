mod functions;
use functions::*;

#[tokio::main]
async fn main() {
    check_option();
    println!("Hello, world!");
}
