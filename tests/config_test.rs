use forward_bot::config::{Config, parse_from_fs};
use std::fs;

#[test]
fn test_multiple_chats_parsing() {
    // Create a test config file
    let test_config = r#"
[bot]
token = "test_token_12345"

[[chats]]
from_id = 123
to_id = 456

[[chats]]
from_id = 789
to_id = 101112

[[chats]]
from_id = 111
to_id = 222

[logging]
dirs = "bot=debug,info"
"#;

    // Write test config to a temporary file
    let test_path = "test_config.toml";
    fs::write(test_path, test_config).expect("Failed to write test config");

    // Parse the config
    let config = parse_from_fs(test_path).expect("Failed to parse config");

    // Assertions
    assert_eq!(config.bot.token.as_ref(), "test_token_12345");
    assert_eq!(config.chats.len(), 3);

    assert_eq!(config.chats[0].from_id, 123);
    assert_eq!(config.chats[0].to_id, 456);

    assert_eq!(config.chats[1].from_id, 789);
    assert_eq!(config.chats[1].to_id, 101112);

    assert_eq!(config.chats[2].from_id, 111);
    assert_eq!(config.chats[2].to_id, 222);

    assert_eq!(config.logging.dirs.as_ref(), "bot=debug,info");

    // Clean up
    fs::remove_file(test_path).ok();
}
