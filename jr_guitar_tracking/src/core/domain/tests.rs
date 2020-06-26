pub struct MethodTest();
impl MethodTest {
    pub fn message(self: &Self) -> String {
        return String::from("message from Tune 2");
    }
}