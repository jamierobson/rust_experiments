#[path = "core/mod.rs"] mod core;
#[path = "infrastructure/mod.rs"] mod infrastructure;
use crate::core::domain;

fn main() {
    let method_test = domain::tests::MethodTest();
    println!("Message: {}", method_test.message());
}
