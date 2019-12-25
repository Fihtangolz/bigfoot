#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SQLParser;

#[test]
fn return_null() {
    parses_to! {
        parser: SQLParser,
        input: "RETURN NULL",
        rule: Rule::return_statement,
        tokens: [
            return_statement(0, 11, [
                return_value(7, 11)
            ])
        ]
    };
}

#[test]
fn return_expr() {
    parses_to! {
        parser: SQLParser,
        input: "RETURN TRUE",
        rule: Rule::return_statement,
        tokens: [
            return_statement(0, 11, [
                value_expression(7, 11)
            ])
        ]
    };
}