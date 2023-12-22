enum Value {
    Int(i32),
    Char(String),
}

impl Value {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Int(v) => v.to_le_bytes().to_vec(),
            Self::Char(s) => s.as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn test_int() {
        let expected = 100;

        let value = Value::Int(expected);
        let bytes = value.as_bytes();

        assert_eq!(expected, i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    #[test]
    fn test_char() -> crate::Result<()> {
        let expected = "hello, value".to_string();

        let value = Value::Char(expected.clone());
        let bytes = value.as_bytes();
        let s = String::from_utf8(bytes)?;

        assert_eq!(expected, s);
        Ok(())
    }
}

