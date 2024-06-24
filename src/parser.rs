pub enum RedisCommand {
    Get(String),
    Set(String, String),
    Unknown,
}

pub fn parse_command(buffer: &[u8]) -> RedisCommand {
    let command = std::str::from_utf8(buffer).unwrap_or("");
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["GET", key] => RedisCommand::Get(key.to_string()),
        ["SET", key, value] => RedisCommand::Set(key.to_string(), value.to_string()),
        _ => RedisCommand::Unknown,
    }
}
