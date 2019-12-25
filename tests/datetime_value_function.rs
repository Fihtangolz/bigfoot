#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SQLParser;

#[test]
fn current_date() {
    parses_to! {
        parser: SQLParser,
        input: "CURRENT_DATE",
        rule: Rule::current_date_value_function,
        tokens: [
            current_date_value_function(0, 12)
        ]
    };
}

#[test]
fn current_time() {
    parses_to! {
        parser: SQLParser,
        input: "CURRENT_TIME",
        rule: Rule::current_time_value_function,
        tokens: [
            current_time_value_function(0, 12)
        ]
    };
}

#[test]
fn current_time_with_arg() {
    parses_to! {
        parser: SQLParser,
        input: "CURRENT_TIME(123)",
        rule: Rule::current_time_value_function,
        tokens: [
            current_time_value_function(0, 17, [
                time_precision(13, 16)
            ])
        ]
    };
}

#[test]
fn localtime() {
    parses_to! {
        parser: SQLParser,
        input: "LOCALTIME",
        rule: Rule::current_local_time_value_function,
        tokens: [
            current_local_time_value_function(0, 9)
        ]
    };
}

#[test]
fn localtime_with_arg() {
    parses_to! {
        parser: SQLParser,
        input: "LOCALTIME(123)",
        rule: Rule::current_local_time_value_function,
        tokens: [
            current_local_time_value_function(0, 14, [
                time_precision(10, 13),
            ])
        ]
    };
}

#[test]
fn current_timestamp() {
    parses_to! {
        parser: SQLParser,
        input: "CURRENT_TIMESTAMP",
        rule: Rule::current_timestamp_value_function,
        tokens: [
            current_timestamp_value_function(0, 17)
        ]
    };
}

#[test]
fn current_timestamp_with_arg() {
    parses_to! {
        parser: SQLParser,
        input: "CURRENT_TIMESTAMP(123)",
        rule: Rule::current_timestamp_value_function,
        tokens: [
            current_timestamp_value_function(0, 22, [
                timestamp_precision(18, 21)
            ])
        ]
    };
}

#[test]
fn localtimestamp() {
    parses_to! {
        parser: SQLParser,
        input: "LOCALTIMESTAMP",
        rule: Rule::current_local_timestamp_value_function,
        tokens: [
            current_local_timestamp_value_function(0, 14)
        ]
    };
}

#[test]
fn localtimestamp_with_arg() {
    parses_to! {
        parser: SQLParser,
        input: "LOCALTIMESTAMP(123)",
        rule: Rule::current_local_timestamp_value_function,
        tokens: [
            current_local_timestamp_value_function(0, 19, [
                timestamp_precision(15, 18),
            ])
        ]
    };
}
