mod functions;
use functions::*;

#[tokio::main]
async fn main() {
    check_option();
    hash_password("e".to_string())
}
