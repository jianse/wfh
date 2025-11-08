use crate::error::{ExportError, Result};

pub(crate) fn get_str<'a>(obj: &'a serde_json::Value, key: &str) -> Result<&'a str> {
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

pub(crate) fn get_opt_str<'a>(obj: &'a serde_json::Value, key: &str) -> Result<Option<&'a str>> {
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

pub(crate) fn get_array<'a>(
    obj: &'a serde_json::Value,
    key: &str,
) -> Result<&'a Vec<serde_json::Value>> {
    obj.as_object()
        .and_then(|obj| obj.get(key))
        .and_then(|value| value.as_array())
        .ok_or_else(|| ExportError::JsonParse(format!("key {} not found", key)))
}

pub(crate) fn get_array_mut<'a>(
    obj: &'a mut serde_json::Value,
    key: &str,
) -> Result<&'a mut Vec<serde_json::Value>> {
    obj.as_object_mut()
        .and_then(|obj| obj.get_mut(key))
        .and_then(|value| value.as_array_mut())
        .ok_or_else(|| ExportError::JsonParse(format!("key {} not found", key)))
}

pub(crate) fn set_value(
    obj: &mut serde_json::Value,
    key: &str,
    value: serde_json::Value,
) -> Result<()> {
    let obj = obj
        .as_object_mut()
        .ok_or(ExportError::JsonParse("not an object".to_string()))?;
    obj.insert(key.to_string(), value);

    Ok(())
}
