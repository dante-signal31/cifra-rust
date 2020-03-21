#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod attack;
pub mod cipher;
pub mod encoding;
mod schema;

#[macro_use]
extern crate diesel_migrations;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
