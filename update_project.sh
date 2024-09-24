#!/bin/bash

# Update package.json
echo "Updating package.json..."
jq '.scripts += {"test:rust": "cargo test"}' package.json > package.json.tmp && mv package.json.tmp package.json

# Update Cargo.toml
echo "Updating Cargo.toml..."
if ! grep -q "\[dev-dependencies\]" Cargo.toml; then
    echo -e "\n[dev-dependencies]\nnapi = { version = \"2.12.0\", features = [\"async\"] }" >> Cargo.toml
fi

# Create tests directory and lib.rs file
echo "Creating tests/lib.rs..."
mkdir -p tests
cat << EOF > tests/lib.rs
use robotjs::RobotJS;
use std::thread;
use std::time::Duration;

#[test]
fn test_mouse_movement() {
    let mut robot = RobotJS::new();

    // Move mouse to a specific position
    robot.move_mouse(100, 100).unwrap();
    thread::sleep(Duration::from_millis(500));

    // Get mouse position
    let pos = robot.get_mouse_pos(napi::Env::default()).unwrap();
    let x: i32 = pos.get("x").unwrap().unwrap();
    let y: i32 = pos.get("y").unwrap().unwrap();

    assert_eq!(x, 100);
    assert_eq!(y, 100);
}

#[test]
fn test_keyboard_input() {
    let mut robot = RobotJS::new();

    // Type a string
    robot.type_string("Hello, RobotJS!".to_string()).unwrap();

    // In a real scenario, you'd need to check the typed text in an input field
    // For this test, we'll just check that the function doesn't panic
}

#[test]
fn test_screen_capture() {
    let robot = RobotJS::new();

    // Capture the entire screen
    let screenshot = robot.capture_screen(napi::Env::default(), None, None, None, None).unwrap();

    // Check that the screenshot has some content
    let width: i32 = screenshot.get("width").unwrap().unwrap();
    let height: i32 = screenshot.get("height").unwrap().unwrap();
    let image_data: Vec<u8> = screenshot.get("image").unwrap().unwrap();

    assert!(width > 0);
    assert!(height > 0);
    assert!(!image_data.is_empty());
}

#[test]
fn test_pixel_color() {
    let robot = RobotJS::new();

    // Get color of a specific pixel
    let color = robot.get_pixel_color(0, 0).unwrap();

    // Check that the color is a valid hex color
    assert!(color.starts_with("#"));
    assert_eq!(color.len(), 7);
}
EOF

# Update update_and_commit.sh
echo "Updating update_and_commit.sh..."
sed -i '' '/git add .cargo\/ .circleci\/ Cargo.toml build.rs src\/lib.rs/a\
git add tests/lib.rs' update_and_commit.sh

echo "All updates completed. Please review the changes and run ./update_and_commit.sh to commit them."