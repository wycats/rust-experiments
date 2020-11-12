pub trait Leaf<T> {
    fn description(&self) -> String;
    fn value(&self) -> T;
}
