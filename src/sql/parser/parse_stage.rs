use crate::sql::parser::parse_defs::{ParsedSqlNode, ParsedSqlResult};

use super::{create, parse_defs};
use log::warn;
use sqlparser::ast::Statement;
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

fn handle_request(query: &str) -> ParsedSqlResult {
    let dialect: MySqlDialect = MySqlDialect {};
    let statements = &Parser::parse_sql(&dialect, query).unwrap();
    println!("============");
    println!("Statement: {:?}", &statements);
    println!("============");

    let mut parsed_sql_result = ParsedSqlResult::new();
    for statement in statements {
        match statement {
            Statement::CreateTable { .. } => match create::handle_create_query(statement) {
                Ok(create_table) => parsed_sql_result
                    .add_sql_node(Box::new(ParsedSqlNode::new_with_create_table(create_table))),
                Err(e) => {
                    warn!("{}", e)
                }
            },
            _ => {
                warn!("cannot parse request!");
            }
        }
    }
    parsed_sql_result
}

#[test]
fn main() {
    println!("经过测试结果如下");
    println!(
        "{:?}",
        handle_request("create table t (id int PRIMARY KEY, name string)")
    )
}
