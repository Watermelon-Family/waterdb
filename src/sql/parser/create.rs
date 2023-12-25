use super::{
    parse_defs::{AttrInfoSqlNode, CreateTableSqlNode},
    value::AttrType,
};
use log::warn;
use sqlparser::ast::{CharacterLength, ColumnOption, DataType, Statement};

pub fn handle_create_query(statement: &Statement) -> Result<CreateTableSqlNode, String> {
    match statement {
        Statement::CreateTable {
            or_replace,
            temporary,
            external,
            global,
            if_not_exists,
            transient,
            name,
            columns,
            constraints,
            hive_distribution,
            hive_formats,
            table_properties,
            with_options,
            file_format,
            location,
            query,
            without_rowid,
            like,
            clone,
            engine,
            comment,
            auto_increment_offset,
            default_charset,
            collation,
            on_cluster,
            on_commit,
            order_by,
            strict,
        } => {
            let relation_name = name.to_string();
            let mut attr_infos: Vec<AttrInfoSqlNode> = vec![];
            for column in columns {
                let attr_name = column.name.to_string();
                let (attr_type, attr_len) = match &column.data_type {
                    DataType::SmallInt(_def_val) => (AttrType::INTS, 4),
                    DataType::Int(_def_val) => (AttrType::INTS, 4),
                    DataType::BigInt(_def_val) => (AttrType::INTS, 4),
                    DataType::Boolean => (AttrType::BOOLEANS, 1),
                    DataType::Text => (AttrType::CHARS, 65535),
                    DataType::Varchar(characterLengthOption) => {
                        let _attr_len = match characterLengthOption {
                            Some(characterLength) => {
                                match characterLength {
                                    CharacterLength::IntegerLength { length, unit } => {
                                        // 在这里使用 length 变量，它包含了 IntegerLength 中的 length 值
                                        length.to_owned()
                                    }
                                    _ => 65535,
                                }
                            }
                            _ => 65535,
                        };
                        (AttrType::CHARS, _attr_len)
                    }
                    DataType::Float(_precision) => (AttrType::FLOATS, 4),
                    DataType::Double => (AttrType::FLOATS, 4),
                    DataType::Decimal(_exactNumberInfo) => (AttrType::FLOATS, 4),
                    DataType::String(_attr_len) => (AttrType::CHARS, _attr_len.unwrap_or(65535)),
                    _ => {
                        warn!("not matched on custom type");
                        (AttrType::UNDEFINED, 1)
                    }
                };
                let mut is_pk: bool = false;
                for column_option in &column.options {
                    is_pk = match column_option.option {
                        ColumnOption::Unique { is_primary } => is_primary,
                        _ => false,
                    };
                }
                attr_infos.push(AttrInfoSqlNode::new(
                    attr_type,
                    attr_name,
                    attr_len as usize,
                    is_pk,
                ))
            }
            Ok(CreateTableSqlNode::new(relation_name, attr_infos))
        }
        _ => Err("Error parsing query".to_string()),
    }
}
