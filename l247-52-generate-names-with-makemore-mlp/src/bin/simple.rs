#[path = "../simple.rs"]
mod simple;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple::main()
}
