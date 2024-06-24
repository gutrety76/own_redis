use std::collections::HashMap;
use serde_json::Value as JsonValue;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub enum DbValue {
    String(String, Option<SystemTime>),
    Int(i32, Option<SystemTime>),
    Json(JsonValue, Option<SystemTime>),
    HashMap(HashMap<String, DbValue>, Option<SystemTime>),
}

impl From<String> for DbValue {
    fn from(value: String) -> Self {
        DbValue::String(value, None)
    }
}

impl From<&str> for DbValue {
    fn from(value: &str) -> Self {
        DbValue::String(value.to_string(), None)
    }
}

impl From<i32> for DbValue {
    fn from(value: i32) -> Self {
        DbValue::Int(value, None)
    }
}

impl From<JsonValue> for DbValue {
    fn from(value: JsonValue) -> Self {
        DbValue::Json(value, None)
    }
}

impl From<HashMap<String, DbValue>> for DbValue {
    fn from(value: HashMap<String, DbValue>) -> Self {
        DbValue::HashMap(value, None)
    }
}


#[derive(Debug)]
pub struct Db {
    values: HashMap<String, DbValue>,
}

impl Db {
    pub fn new() -> Self {
        Db {
            values: HashMap::new(),
        }
    }

    pub fn set<V: Into<DbValue>>(&mut self, key: String, value: V, ttl: Option<Duration>) -> Result<String, String> {
        let expiration = ttl.map(|dur| SystemTime::now() + dur);
        let db_value = match value.into() {
            DbValue::String(v, _) => DbValue::String(v, expiration),
            DbValue::Int(v, _) => DbValue::Int(v, expiration),
            DbValue::Json(v, _) => DbValue::Json(v, expiration),
            DbValue::HashMap(v, _) => DbValue::HashMap(v, expiration),
        };
        self.values.insert(key, db_value);
        Ok("OK\n".to_string())
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        match self.values.get(key) {
            Some(DbValue::String(value, expiration)) => {
                if Self::is_expired(expiration) {
                    Err("Key not found or expired\n".to_string())
                } else {
                    Ok(format!("{}\n", value))
                }
            },
            Some(DbValue::Int(value, expiration)) => {
                if Self::is_expired(expiration) {
                    Err("Key not found or expired\n".to_string())
                } else {
                    Ok(format!("{}\n", value))
                }
            },
            Some(DbValue::Json(value, expiration)) => {
                if Self::is_expired(expiration) {
                    Err("Key not found or expired\n".to_string())
                } else {
                    Ok(format!("{}\n", value))
                }
            },
            Some(DbValue::HashMap(value, expiration)) => {
                if Self::is_expired(expiration) {
                    Err("Key not found or expired\n".to_string())
                } else {
                    Ok(format!("{:?}\n", value))
                }
            },
            None => Err("Key not found\n".to_string()),
        }
    }

    fn is_expired(expiration: &Option<SystemTime>) -> bool {
        match expiration {
            Some(exp) => {
                match SystemTime::now().duration_since(*exp) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            },
            None => false,
        }
    }

    pub fn remove_expired(&mut self) {
        let now = SystemTime::now();
        self.values.retain(|_, value| {
            match value {
                DbValue::String(_, exp) => exp.map_or(true, |e| e > now),
                DbValue::Int(_, exp) => exp.map_or(true, |e| e > now),
                DbValue::Json(_, exp) => exp.map_or(true, |e| e > now),
                DbValue::HashMap(_, exp) => exp.map_or(true, |e| e > now),
            }
        });
    }
}
