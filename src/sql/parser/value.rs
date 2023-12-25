use log::warn;
use std::{fmt, str::FromStr};

// 属性的类型
#[derive(Debug, PartialEq, Eq)]
pub enum AttrType {
    UNDEFINED, // 为定义类型(默认1字节)
    CHARS,     // 字符串类型(默认65535字节)
    INTS,      // 整数类型(4字节)
    FLOATS,    // 浮点数类型(4字节)
    BOOLEANS,  // boolean类型(1字节)
}

impl AttrType {
    fn attr_type_to_string(&self) -> &'static str {
        match self {
            AttrType::UNDEFINED => "undefined",
            AttrType::CHARS => "chars",
            AttrType::INTS => "ints",
            AttrType::FLOATS => "floats",
            AttrType::BOOLEANS => "booleans",
        }
    }
}
fn attr_type_from_string(s: &str) -> AttrType {
    match s {
        "chars" => AttrType::CHARS,
        "ints" => AttrType::INTS,
        "floats" => AttrType::FLOATS,
        "booleans" => AttrType::BOOLEANS,
        _ => AttrType::UNDEFINED,
    }
}

// 属性的值
#[derive(Debug, PartialEq)]
pub struct Value {
    attr_type: AttrType,
    length: usize,
    num_value: NumValue,
    str_value: String,
}

#[derive(Debug, PartialEq)]
enum NumValue {
    IntValue(i32),
    FloatValue(f32),
    BoolValue(bool),
}

impl fmt::Display for NumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumValue::IntValue(value) => write!(f, "{}", value),
            NumValue::FloatValue(value) => write!(f, "{}", value),
            NumValue::BoolValue(value) => write!(f, "{}", value),
        }
    }
}

impl Value {
    pub fn new() -> Self {
        Self {
            attr_type: AttrType::UNDEFINED,
            length: 0,
            num_value: NumValue::IntValue(0),
            str_value: String::new(),
        }
    }

    pub fn set_type(&mut self, attr_type: AttrType) {
        self.attr_type = attr_type;
    }

    pub fn set_data(&mut self, data: &str, length: usize) {
        match self.attr_type {
            AttrType::CHARS => {
                self.str_value = data.to_string();
                if length > 0 {
                    self.length = length;
                } else {
                    self.length = self.str_value.len();
                }
            }
            AttrType::INTS => {
                let int_value = i32::from_str(data).unwrap_or(0);
                self.num_value = NumValue::IntValue(int_value);
                self.length = length;
            }
            AttrType::FLOATS => {
                let float_value = f32::from_str(data).unwrap_or(0.0);
                self.num_value = NumValue::FloatValue(float_value);
                self.length = length;
            }
            AttrType::BOOLEANS => {
                let bool_value = bool::from_str(data).unwrap_or(false);
                self.num_value = NumValue::BoolValue(bool_value);
                self.length = length;
            }
            _ => {
                warn!("unknown data type: {:?}", self.attr_type);
            }
        }
    }

    pub fn set_int(&mut self, val: i32) {
        self.attr_type = AttrType::INTS;
        self.num_value = NumValue::IntValue(val);
        self.length = std::mem::size_of::<i32>();
    }

    pub fn set_float(&mut self, val: f32) {
        self.attr_type = AttrType::FLOATS;
        self.num_value = NumValue::FloatValue(val);
        self.length = std::mem::size_of::<f32>();
    }

    pub fn set_boolean(&mut self, val: bool) {
        self.attr_type = AttrType::BOOLEANS;
        self.num_value = NumValue::BoolValue(val);
        self.length = std::mem::size_of::<bool>();
    }

    pub fn set_string(&mut self, s: &str) {
        self.attr_type = AttrType::CHARS;
        self.str_value = s.to_string();
        self.length = self.str_value.len();
    }

    pub fn to_string(&self) -> String {
        match &self.attr_type {
            AttrType::INTS => self.num_value.to_string(),
            AttrType::FLOATS => self.num_value.to_string(),
            AttrType::BOOLEANS => self.num_value.to_string(),
            AttrType::CHARS => self.str_value.clone(),
            _ => {
                warn!("unsupported attr type: {:?}", self.attr_type);
                String::from("")
            }
        }
    }

    pub fn compare(&self, other: &Value) -> i32 {
        if self.attr_type == other.attr_type {
            match (&self.attr_type, &self.num_value, &other.num_value) {
                (AttrType::INTS, NumValue::IntValue(val1), NumValue::IntValue(val2)) => {
                    val1.cmp(val2) as i32
                }
                (AttrType::FLOATS, NumValue::FloatValue(val1), NumValue::FloatValue(val2)) => {
                    val1.partial_cmp(val2).unwrap() as i32
                }
                (AttrType::BOOLEANS, NumValue::BoolValue(val1), NumValue::BoolValue(val2)) => {
                    val1.cmp(val2) as i32
                }
                (AttrType::CHARS, _, _) => self.str_value.cmp(&other.str_value) as i32,
                _ => {
                    warn!("not supported");
                    -1
                }
            }
        } else if let NumValue::FloatValue(val2) = &other.num_value {
            if let NumValue::IntValue(val1) = &self.num_value {
                (&(*val1 as f32)).partial_cmp(val2).unwrap() as i32
            } else {
                warn!("not supported");
                -1
            }
        } else if let NumValue::IntValue(val1) = &other.num_value {
            if let NumValue::FloatValue(val2) = &self.num_value {
                val2.partial_cmp(&(*val1 as f32)).unwrap() as i32
            } else {
                warn!("not supported");
                -1
            }
        } else {
            warn!("not supported");
            -1
        }
    }

    pub fn get_int(&self) -> i32 {
        match &self.num_value {
            NumValue::IntValue(val) => *val,
            _ => 0,
        }
    }

    pub fn get_float(&self) -> f32 {
        match &self.num_value {
            NumValue::FloatValue(val) => *val,
            _ => 0.0,
        }
    }

    pub fn get_string(&self) -> String {
        match &self.attr_type {
            AttrType::CHARS => self.str_value.clone(),
            _ => String::from("unsupported"),
        }
    }

    pub fn get_boolean(&self) -> bool {
        match &self.attr_type {
            AttrType::CHARS => match &self.str_value.parse::<i32>() {
                Ok(val) => val != &0,
                Err(e) => {
                    warn!(
                        "failed to convert string to float or integer. s={:?}, ex={:?}",
                        &self.str_value,
                        e.clone()
                    );
                    false
                }
            },
            _ => match &self.num_value {
                NumValue::IntValue(val) => val != &0,
                NumValue::FloatValue(val) => val != &0.0,
                NumValue::BoolValue(val) => *val,
            },
        }
    }
}

#[test]
fn test() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Warn)
        .init();

    // Example usage
    let mut value1 = Value::new();
    let mut value2 = Value::new();
    value1.set_int(42);
    println!("Int value: {}", value1.get_int());

    value2.set_boolean(true);
    println!("Bool value: {}", value2.get_boolean());

    // value.set_boolean(true);
    // println!("Boolean value: {}", value.get_boolean());

    // value.set_string("Hello, Rust!");
    // println!("String value: {}", value.get_string());

    println!("{:?}", value1.compare(&value2))
}
