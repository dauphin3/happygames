

pub mod login;

pub trait UserAction {
    fn new(&self) -> Self;
    fn fetch(&self) -> Self;
}