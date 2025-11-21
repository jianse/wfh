use serde_json::Map;

use crate::error::{ExportError, Result};

pub type JsonObject = Map<String, serde_json::Value>;

pub fn get_path<'a>(obj: &'a serde_json::Value, path: &str) -> Result<&'a serde_json::Value> {
    let path = path.split('.');
    let mut obj = obj;
    for part in path {
        obj = obj
            .get(part)
            .ok_or(ExportError::JsonParse(format!("key {} not found", part)))?;
    }

    Ok(obj)
}

pub fn get_object_mut<'a>(obj: &'a mut serde_json::Value, key: &str) -> Result<&'a mut JsonObject> {
    let obj = obj
        .as_object_mut()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj
        .get_mut(key)
        .ok_or(ExportError::JsonParse(format!("key {} not found", key)))?;
    let out = field
        .as_object_mut()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    Ok(out)
}

pub fn get_mut<'a>(obj: &'a mut serde_json::Value, key: &str) -> Result<&'a mut serde_json::Value> {
    let obj = obj
        .as_object_mut()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj
        .get_mut(key)
        .ok_or(ExportError::JsonParse(format!("key {} not found", key)))?;
    Ok(field)
}

pub fn get_str<'a>(obj: &'a serde_json::Value, key: &str) -> Result<&'a str> {
    let obj = obj
        .as_object()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj
        .get(key)
        .ok_or(ExportError::JsonParse(format!("key {} not found", key)))?;
    field
        .as_str()
        .ok_or(ExportError::JsonParse("not a string".to_string()))
}

pub fn get_opt_str<'a>(obj: &'a serde_json::Value, key: &str) -> Result<Option<&'a str>> {
    let obj = obj
        .as_object()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj.get(key);
    match field {
        Some(field) => field
            .as_str()
            .ok_or(ExportError::JsonParse("not a string".to_string()))
            .map(Some),
        None => Ok(None),
    }
}

pub fn get_array<'a>(obj: &'a serde_json::Value, key: &str) -> Result<&'a Vec<serde_json::Value>> {
    obj.as_object()
        .and_then(|obj| obj.get(key))
        .and_then(|value| value.as_array())
        .ok_or_else(|| ExportError::JsonParse(format!("key {} not found", key)))
}

pub fn get_opt_array<'a>(
    obj: &'a serde_json::Value,
    key: &str,
) -> Result<Option<&'a Vec<serde_json::Value>>> {
    let obj = obj
        .as_object()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj.get(key);
    match field {
        Some(field) => field
            .as_array()
            .ok_or(ExportError::JsonParse("not an array".to_string()))
            .map(Some),
        None => Ok(None),
    }
}

pub fn get_array_mut<'a>(
    obj: &'a mut serde_json::Value,
    key: &str,
) -> Result<&'a mut Vec<serde_json::Value>> {
    obj.as_object_mut()
        .and_then(|obj| obj.get_mut(key))
        .and_then(|value| value.as_array_mut())
        .ok_or_else(|| ExportError::JsonParse(format!("key {} not found", key)))
}

pub fn set_value(obj: &mut serde_json::Value, key: &str, value: serde_json::Value) -> Result<()> {
    let obj = obj
        .as_object_mut()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    obj.insert(key.to_string(), value);

    Ok(())
}

pub fn get_opt_u64(obj: &serde_json::Value, key: &str) -> Result<Option<u64>> {
    let obj = obj
        .as_object()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj.get(key);
    match field {
        Some(field) => field
            .as_u64()
            .ok_or(ExportError::JsonParse("not a u64".to_string()))
            .map(Some),
        None => Ok(None),
    }
}

pub fn get_u64(obj: &serde_json::Value, key: &str) -> Result<u64> {
    let obj = obj
        .as_object()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    let field = obj
        .get(key)
        .ok_or(ExportError::JsonParse(format!("key {} not found", key)))?;
    field
        .as_u64()
        .ok_or(ExportError::JsonParse("not a u64".to_string()))
}
