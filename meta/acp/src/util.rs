use toml::{Table, Value};

/// Merge `b` into `a` recursively
/// - Do not allow changing of types
/// - Deep merge tables
pub fn deep_merge_table(a: &mut Table, b: &Table, path: String) {
    for (b_key, b_val) in b {
        match (a.get_mut(b_key), b_val) {
            (Some(a_val), b_val) => match (a_val, b_val) {
                (Value::String(_), Value::String(b_val)) => {
                    a.insert(b_key.clone(), Value::String(b_val.to_owned()));
                }
                (Value::Integer(_), Value::Integer(b_val)) => {
                    a.insert(b_key.clone(), Value::Integer(*b_val));
                }
                (Value::Float(_), Value::Float(b_val)) => {
                    a.insert(b_key.clone(), Value::Float(*b_val));
                }
                (Value::Boolean(_), Value::Boolean(b_val)) => {
                    a.insert(b_key.clone(), Value::Boolean(*b_val));
                }
                (Value::Datetime(_), Value::Datetime(b_val)) => {
                    a.insert(b_key.clone(), Value::Datetime(b_val.to_owned()));
                }
                (Value::Array(_), Value::Array(b_val)) => {
                    a.insert(b_key.clone(), Value::Array(b_val.to_owned()));
                }
                (Value::Table(a_val), Value::Table(b_val)) => {
                    deep_merge_table(a_val, b_val, path.clone() + "." + b_key);
                }
                (a_val, b_val) => {
                    panic!("Incompatible types at `{}.{}`\n - original value: {}\n - incoming value: {}", path, b_key, a_val, b_val);
                }
            },
            (None, value) => {
                a.insert(b_key.clone(), value.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {}
