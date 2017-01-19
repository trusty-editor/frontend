extern crate trusty_utils;
use trusty_utils::*;

pub trait FrontEnd {
    fn new() -> Self;
    fn poll_for_event(&self) -> event::Event;
    fn render(&self, &cursor::Cursor, &str);
    fn request_user_input(&self, &str) -> &str; 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
