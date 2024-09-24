use napi::bindgen_prelude::*;
use napi::NapiValue;
use std::thread;
use std::time::Duration;

use robotjs::RobotJS;

struct MockEnv;

impl MockEnv {
    fn create_object(&self) -> napi::Result<Object> {
        unsafe { Ok(Object::from_raw_unchecked(std::ptr::null_mut(), std::ptr::null_mut())) }
    }
}

impl AsRef<napi::Env> for MockEnv {
    fn as_ref(&self) -> &napi::Env {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<&MockEnv> for Env {
    fn from(_: &MockEnv) -> Self {
        unsafe { std::mem::transmute(std::ptr::null_mut::<std::ffi::c_void>()) }
    }
}

const MOUSE_MOVE_DELAY: Duration = Duration::from_millis(500);
const TEST_MOUSE_X: i32 = 100;
const TEST_MOUSE_Y: i32 = 100;
const TEST_STRING: &str = "Hello, RobotJS!";

#[test]
fn test_mouse_movement() {
    let mut robot = RobotJS::new();

    robot.move_mouse(TEST_MOUSE_X, TEST_MOUSE_Y).unwrap();
    thread::sleep(MOUSE_MOVE_DELAY);

    let mock_env = MockEnv;
    let pos = robot.get_mouse_pos((&mock_env).into()).unwrap();
    let x: i32 = pos.get::<_, i32>("x").unwrap().unwrap();
    let y: i32 = pos.get::<_, i32>("y").unwrap().unwrap();

    assert_eq!(x, TEST_MOUSE_X);
    assert_eq!(y, TEST_MOUSE_Y);
}

#[test]
fn test_keyboard_input() {
    let mut robot = RobotJS::new();
    robot.type_string(TEST_STRING.to_string()).unwrap();
}

#[test]
fn test_screen_capture() {
    let robot = RobotJS::new();
    let mock_env = MockEnv;
    let screenshot = robot.capture_screen((&mock_env).into(), None, None, None, None).unwrap();

    let width: i32 = screenshot.get::<_, i32>("width").unwrap().unwrap();
    let height: i32 = screenshot.get::<_, i32>("height").unwrap().unwrap();
    let image_data: Vec<u8> = screenshot.get::<_, Vec<u8>>("image").unwrap().unwrap();

    assert!(width > 0);
    assert!(height > 0);
    assert!(!image_data.is_empty());
}

#[test]
fn test_pixel_color() {
    let robot = RobotJS::new();
    let color = robot.get_pixel_color(0, 0).unwrap();

    assert!(color.starts_with('#'));
    assert_eq!(color.len(), 7);
}
