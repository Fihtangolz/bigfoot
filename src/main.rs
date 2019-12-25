#[macro_use]
extern crate pest_derive;

mod parser;

fn main() {
    parser::parse_sql("CALL routine_invocation");

}
