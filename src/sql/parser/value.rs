use log::{debug, error, info, trace, warn};
use std::str::FromStr;

// 属性的类型
#[derive(Debug)]
pub enum AttrType {
    UNDEFINED,
    CHARS,    // 字符串类型
    INTS,     // 整数类型(4字节)
    FLOATS,   // 浮点数类型(4字节)
    BOOLEANS, // boolean类型，当前不是由parser解析出来的，是程序内部使用的
}

impl AttrType {
    fn attr_type_to_string(&self) -> &'static str {
        match self {
            AttrType::UNDEFINED => "undefined",
            AttrType::CHARS => "chars",
            AttrType::INTS => "ints",
            AttrType::FLOATS => "floats",
            AttrType::BOOLEANS => "booleans",
            _ => "unknown",
        }
    }
}

impl FromStr for AttrType {
    type Err = ();

    fn attr_type_from_string(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undefined" => Ok(AttrType::UNDEFINED),
            "chars" => Ok(AttrType::CHARS),
            "ints" => Ok(AttrType::INTS),
            "floats" => Ok(AttrType::FLOATS),
            "booleans" => Ok(AttrType::BOOLEANS),
            _ => Err(()),
        }
    }
}

// 属性的值
#[derive(Debug)]
pub struct Value {
    attr_type: AttrType,
    length: usize,
    num_value: NumValue,
    str_value: String,
}

#[derive(Debug)]
enum NumValue {
    IntValue(i32),
    FloatValue(f32),
    BoolValue(bool),
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
                if len > 0 {
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
                info!("AttrType Error in func set_data()");
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
                info!("unsupported attr type: {:?}", self.attr_type);
                String::from("unsupported")
            }
        }
    }

    pub fn compare(&self, other: &Value) -> i32 {
        if self.attr_type == other.attr_type {
            match (&self.attr_type, &self.num_value, &other.num_value) {
                (AttrType::INTS, NumValue::IntValue(val1), NumValue::IntValue(val2)) => {
                    val1.cmp(val2)
                }
                (AttrType::FLOATS, NumValue::FloatValue(val1), NumValue::FloatValue(val2)) => {
                    val1.partial_cmp(val2).unwrap_or(0)
                }
                (AttrType::BOOLEANS, NumValue::BoolValue(val1), NumValue::BoolValue(val2)) => {
                    val1.cmp(val2)
                }
                (AttrType::CHARS, _, _) => self.str_value.cmp(&other.str_value),
                _ => {
                    // Handle other cases if needed
                    0
                }
            }
        } else if self.attr_type == AttrType::INTS && other.attr_type == AttrType::FLOATS {
            if let NumValue::FloatValue(val2) = &other.num_value {
                if let NumValue::IntValue(val1) = &self.num_value {
                    val1.partial_cmp(val2).unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        } else if self.attr_type == AttrType::FLOATS && other.attr_type == AttrType::INTS {
            if let NumValue::IntValue(val1) = &self.num_value {
                if let NumValue::FloatValue(val2) = &other.num_value {
                    val1.partial_cmp(val2).unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            // Handle other cases if needed
            0
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
        match &self.AttrType {
            AttrType::INTS => self.num_value.cmp(0),
            AttrType::FLOATS => self.num_value.cmp(0),
            AttrType::CHARS => self.num_value.cmp(0),
            AttrType::BOOLEANS => self.str_value.is_empty(),
        }
    }
}

fn main() {
    // Example usage
    let mut value = Value::new();
    value.set_int(42);
    println!("Int value: {}", value.get_int());

    value.set_float(3.14);
    println!("Float value: {}", value.get_float());

    value.set_boolean(true);
    println!("Boolean value: {}", value.get_boolean());

    value.set_string("Hello, Rust!");
    println!("String value: {}", value.get_string());
}
