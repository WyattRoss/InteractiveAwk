pub trait Provider {
    fn get(&self) -> String;
}
