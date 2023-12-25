use tracing_subscriber::field::debug;

use super::value::{AttrType, Value};

#[derive(Debug)]
pub enum SqlCommandFlag {
    ScfSowTables,
    ScfCreateTable,
    ScfSelect,
    ScfInsert,
}
#[derive(Debug)]
pub enum CompOp {
    EqualTO,
    LessEQUAL,
    NotEQUAL,
    LessTHAN,
    GreatEqual,
    GreatThan,
    NoOp,
}

#[derive(Debug)]
pub struct AttrInfoSqlNode {
    attr_type: AttrType,
    name: String,
    length: usize, //
    is_pk: bool,
}
impl AttrInfoSqlNode {
    pub fn new(attr_type: AttrType, name: String, length: usize, is_pk: bool) -> Self {
        AttrInfoSqlNode {
            attr_type,
            name,
            length,
            is_pk,
        }
    }
}

#[derive(Default, Debug)]
pub struct RelAttrSqlNode {
    relation_name: String,
    attribute_name: String,
}

#[derive(Default, Debug)]
pub struct CreateTableSqlNode {
    relation_name: String,
    attr_infos: Vec<AttrInfoSqlNode>,
}
impl CreateTableSqlNode {
    pub fn new(relation_name: String, attr_infos: Vec<AttrInfoSqlNode>) -> Self {
        CreateTableSqlNode {
            relation_name,
            attr_infos,
        }
    }
}
#[derive(Debug)]
pub struct ConditionSqlNode {
    left_is_attr: i32,
    left_value: Value,
    left_attr: RelAttrSqlNode,
    comp: CompOp,
    right_is_attr: i32,
    right_attr: RelAttrSqlNode,
    right_value: Value,
}

#[derive(Default, Debug)]
pub struct SelectSqlNode {
    attributes: Vec<RelAttrSqlNode>,
    relations: Vec<String>,
    conditions: Vec<ConditionSqlNode>,
}

#[derive(Default, Debug)]
pub struct InsertSqlNode {
    relation_name: String,
    values: Vec<Value>,
}

#[derive(Debug)]
pub struct ParsedSqlNode {
    flag: SqlCommandFlag,
    create_table: CreateTableSqlNode,
    selection: SelectSqlNode,
    insertion: InsertSqlNode,
}
impl ParsedSqlNode {
    pub fn new(
        flag: SqlCommandFlag,
        create_table: CreateTableSqlNode,
        selection: SelectSqlNode,
        insertion: InsertSqlNode,
    ) -> Self {
        ParsedSqlNode {
            flag,
            create_table,
            selection,
            insertion,
        }
    }
    pub fn new_with_create_table(create_table: CreateTableSqlNode) -> Self {
        ParsedSqlNode::new(
            SqlCommandFlag::ScfCreateTable,
            create_table,
            SelectSqlNode::default(),
            InsertSqlNode::default(),
        )
    }
    pub fn new_with_selection(selection: SelectSqlNode) -> Self {
        ParsedSqlNode::new(
            SqlCommandFlag::ScfCreateTable,
            CreateTableSqlNode::default(),
            selection,
            InsertSqlNode::default(),
        )
    }
    pub fn new_with_insertion(insertion: InsertSqlNode) -> Self {
        ParsedSqlNode::new(
            SqlCommandFlag::ScfCreateTable,
            CreateTableSqlNode::default(),
            SelectSqlNode::default(),
            insertion,
        )
    }
}

#[derive(Debug)]
pub struct ParsedSqlResult {
    sql_nodes: Vec<Box<ParsedSqlNode>>,
}
impl ParsedSqlResult {
    pub fn new() -> Self {
        ParsedSqlResult {
            sql_nodes: Vec::new(),
        }
    }
    pub fn add_sql_node(&mut self, sql_node: Box<ParsedSqlNode>) {
        self.sql_nodes.push(sql_node);
    }
    pub fn sql_nodes(&self) -> &Vec<Box<ParsedSqlNode>> {
        &self.sql_nodes
    }
}
