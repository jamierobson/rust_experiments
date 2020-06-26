pub trait DataReader<T> {
    fn list() -> Vec<T>;
    fn get(id: u32) -> T;
}