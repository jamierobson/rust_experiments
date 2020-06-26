pub trait DataWriter<T> {
    fn create(item: T);
    fn replace(id: u32, item: T);
    fn delete(id: u32);
}