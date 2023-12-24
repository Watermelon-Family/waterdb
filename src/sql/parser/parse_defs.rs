use super::value::{AttrType, Value};

pub enum SqlCommandFlag {
    ScfSowTables,
    ScfCreateTable,
    ScfSelect,
    ScfInsert,
}
pub enum CompOp {
    EqualTO,
    LessEQUAL,
    NotEQUAL,
    LessTHAN,
    GreatEqual,
    GreatThan,
    NoOp,
}

pub struct AttrInfoSqlNode {
    attr_type: AttrType,
    name: String,
    length: usize,
}
pub struct RelAttrSqlNode {
    relation_name: String,
    attribute_name: String,
}

pub struct CreateTableSqlNode {
    relation_name: String,
    attr_infos: Vec<AttrInfoSqlNode>,
}
pub struct ConditionSqlNode {
    left_is_attr: i32,
    left_value: Value,
    left_attr: RelAttrSqlNode,
    comp: CompOp,
    right_is_attr: i32,
    right_attr: RelAttrSqlNode,
    right_value: Value,
}

pub struct SelectSqlNode {
    attributes: Vec<RelAttrSqlNode>,
    relations: Vec<String>,
    conditions: Vec<ConditionSqlNode>,
}

pub struct InsertSqlNode {
    relation_name: String,
    values: Vec<Value>,
}

pub struct ParsedSqlNode {
    flag: SqlCommandFlag,
    create_table: CreateTableSqlNode,
    selection: SelectSqlNode,
    insertion: InsertSqlNode,
}

pub struct ParsedSqlResult {
    sql_nodes: Vec<Box<ParsedSqlNode>>,
}

impl ParsedSqlResult {
    fn new() -> Self {
        ParsedSqlResult {
            sql_nodes: Vec::new(),
        }
    }
    fn add_sql_node(&mut self, sql_node: Box<ParsedSqlNode>) {
        self.sql_nodes.push(sql_node);
    }
    fn sql_nodes(&self) -> &Vec<Box<ParsedSqlNode>> {
        &self.sql_nodes
    }
}
