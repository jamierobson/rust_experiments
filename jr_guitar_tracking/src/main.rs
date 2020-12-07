#[path = "core/domain.rs"] mod domain;

fn main() {
    let method_test = domain::MethodTest();
    println!("Message: {}", method_test.message());
}
