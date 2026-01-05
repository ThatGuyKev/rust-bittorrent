use std::io::Cursor;

use bytes::Buf;

pub fn decode_string(encoded_str: &str) -> (serde_json::Value, &str) {
    let col_pos = encoded_str.find(':').unwrap();
    let str_len = encoded_str[..col_pos].parse::<usize>().unwrap();
    (
        serde_json::Value::String(encoded_str[col_pos + 1..col_pos + 1 + str_len].to_string()),
        &encoded_str[col_pos + 1 + str_len..],
    )
}

pub fn decode_integer(encoded_str: &str) -> (serde_json::Value, &str) {
    let (_, rest) = encoded_str.split_at(1);
    let end_pos = rest.find('e').unwrap();
    let int_str = &rest[..end_pos];
    let int_value = int_str.parse::<i64>().unwrap();
    (
        serde_json::Value::Number(serde_json::Number::from(int_value)),
        &rest[end_pos + 1..],
    )
}

pub fn decode_list(encoded_str: &str) -> (serde_json::Value, &str) {
    let mut values = Vec::new();
    let (_, mut rest) = encoded_str.split_at(1);
    while !rest.is_empty() && !rest.starts_with('e') {
        let (v, remainder) = decode_bencoded_value(rest);
        values.push(v);
        rest = remainder;
    }
    (values.into(), &rest[1..])
}

pub fn decode_dictionary(encoded_str: &str) -> (serde_json::Value, &str) {
    let mut map = serde_json::Map::new();
    let (_, mut rest) = encoded_str.split_at(1);
    while !rest.is_empty() && !rest.starts_with('e') {
        let (key, remainder_with_value) = decode_bencoded_value(rest);
        let (v, remainder) = decode_bencoded_value(remainder_with_value);
        map.insert(key.to_string(), v);
        rest = remainder;
    }

    (serde_json::Value::Object(map), &rest[1..])
}

#[allow(dead_code)]
pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    // If encoded_value starts with a digit, it's a number
    let first_char = encoded_value.chars().next().unwrap();
    match first_char {
        'l' => decode_list(encoded_value),
        'd' => decode_dictionary(encoded_value),
        'i' => decode_integer(encoded_value),
        '0'..='9' => decode_string(encoded_value),

        _ => (serde_json::Value::Null, encoded_value),
    }
}

pub fn url_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(3 * bytes.len());
    for &b in bytes {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[b]));
    }
    encoded
}

pub fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, anyhow::Error> {
    if !src.has_remaining() {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(src.get_u8())
}

pub fn get_len(src: &mut Cursor<&[u8]>) -> Result<u32, anyhow::Error> {
    if src.remaining() < 4 {
        return Err(anyhow::anyhow!("Incomplete"));
    }
    Ok(src.get_u32())
}

pub fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, anyhow::Error> {
    if !src.has_remaining() {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(src.chunk()[0])
}

pub fn get_payload(src: &mut Cursor<&[u8]>, length: usize) -> Result<Vec<u8>, anyhow::Error> {
    if src.remaining() < length {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    let mut payload = vec![0; length];
    src.copy_to_slice(&mut payload);

    Ok(payload)
}
