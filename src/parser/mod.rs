extern crate pest;

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SQLParser;

pub mod ast;
use self::ast::*;


fn parse_table_definition() {

}

pub fn parse_sql(sql: &str) -> SQLStmt { //Result<SQLStmt, Error<Rule>>
    let pair = SQLParser::parse(Rule::sql_statements, sql).unwrap().next().unwrap();
    
    match pair.as_rule() {
        Rule::sql_control_statements => {
            let mut inner_pair = pair.into_inner();
            let sub_rule = inner_pair.next().unwrap().as_rule();
            match sub_rule {
                Rule::call_statement => SQLStmt::ControlStmt(ControlStmt::CallStmt(RoutineInvocation{})),
                Rule::return_statement => SQLStmt::ControlStmt(ControlStmt::ReturnStmt(ReturnStmt::NULL)),
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}