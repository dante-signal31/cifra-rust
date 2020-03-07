pub mod attack;
pub mod cipher;
pub mod encoding;

#[macro_use]
extern crate diesel_migrations;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
