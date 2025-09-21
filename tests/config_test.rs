use forward_bot::config::{parse_from_fs, ToIdKind};
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
    assert!(matches!(config.chats[0].to_id, ToIdKind::Single(456)));

    assert_eq!(config.chats[1].from_id, 789);
    assert!(matches!(config.chats[1].to_id, ToIdKind::Single(101112)));

    assert_eq!(config.chats[2].from_id, 111);
    assert!(matches!(config.chats[2].to_id, ToIdKind::Single(222)));

    assert_eq!(config.logging.dirs.as_ref(), "bot=debug,info");

    // Clean up
    fs::remove_file(test_path).ok();
}

#[test]
fn test_one_to_many_chats_parsing() {
    // Create a test config file with one-to-many forwarding
    let test_config = r#"
[bot]
token = "test_token_one_to_many"

[[chats]]
from_id = 100
to_id = [200, 300, 400]

[[chats]]
from_id = 500
to_id = 600

[[chats]]
from_id = 700
to_id = [800, 900]

[logging]
dirs = "bot=debug"
"#;

    // Write test config to a temporary file
    let test_path = "test_one_to_many_config.toml";
    fs::write(test_path, test_config).expect("Failed to write test config");

    // Parse the config
    let config = parse_from_fs(test_path).expect("Failed to parse config");

    // Assertions
    assert_eq!(config.bot.token.as_ref(), "test_token_one_to_many");
    assert_eq!(config.chats.len(), 3);

    // First chat: one-to-many (3 destinations)
    assert_eq!(config.chats[0].from_id, 100);
    match &config.chats[0].to_id {
        ToIdKind::Multiple(ids) => {
            assert_eq!(ids.len(), 3);
            assert_eq!(ids[0], 200);
            assert_eq!(ids[1], 300);
            assert_eq!(ids[2], 400);
        }
        _ => panic!("Expected Multiple variant for first chat config"),
    }

    // Second chat: one-to-one (backward compatible)
    assert_eq!(config.chats[1].from_id, 500);
    assert!(matches!(config.chats[1].to_id, ToIdKind::Single(600)));

    // Third chat: one-to-many (2 destinations)
    assert_eq!(config.chats[2].from_id, 700);
    match &config.chats[2].to_id {
        ToIdKind::Multiple(ids) => {
            assert_eq!(ids.len(), 2);
            assert_eq!(ids[0], 800);
            assert_eq!(ids[1], 900);
        }
        _ => panic!("Expected Multiple variant for third chat config"),
    }

    assert_eq!(config.logging.dirs.as_ref(), "bot=debug");

    // Clean up
    fs::remove_file(test_path).ok();
}

#[test]
fn test_to_id_kind_as_slice() {
    // Test Single variant
    let single = ToIdKind::Single(100);
    let slice = single.as_slice();
    assert_eq!(slice.len(), 1);
    assert_eq!(slice[0], 100);

    // Test Multiple variant
    let multiple = ToIdKind::Multiple(vec![200, 300, 400]);
    let slice = multiple.as_slice();
    assert_eq!(slice.len(), 3);
    assert_eq!(slice[0], 200);
    assert_eq!(slice[1], 300);
    assert_eq!(slice[2], 400);
}
