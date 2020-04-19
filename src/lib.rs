#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod attack;
pub mod cipher;
// pub mod errors;
pub mod encoding;
mod schema;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate error_chain;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
    errors {
            ConversionError(var: &'static str, var_type: &'static str, tried_type: &'static str) {
                description("Conversion failed")
                display("{} type variable '{}' could not converted to {}", var_type, var, tried_type)
            }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
