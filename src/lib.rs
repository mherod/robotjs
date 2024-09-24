use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::time::Duration;
use enigo::{Enigo, MouseControllable, KeyboardControllable, MouseButton, Key};
use screenshots::Screen;
use napi::{Env, JsObject};

#[napi]
pub struct RobotJS {
    mouse_delay: u64,
    keyboard_delay: u64,
    enigo: Enigo,
}

#[napi]
impl RobotJS {
    #[napi(constructor)]
    pub fn new() -> Self {
        RobotJS {
            mouse_delay: 10,
            keyboard_delay: 10,
            enigo: Enigo::new(),
        }
    }

    #[napi]
    pub fn drag_mouse(&mut self, x: i32, y: i32, button: Option<String>) -> Result<i32> {
        let button = match button.as_deref() {
            Some("left") => MouseButton::Left,
            Some("right") => MouseButton::Right,
            Some("middle") => MouseButton::Middle,
            _ => MouseButton::Left,
        };
        self.enigo.mouse_down(button);
        self.enigo.mouse_move_to(x, y);
        self.enigo.mouse_up(button);
        std::thread::sleep(Duration::from_millis(self.mouse_delay));
        Ok(1)
    }

    #[napi]
    pub fn update_screen_metrics(&self) -> Result<i32> {
        // This is a no-op in Rust implementation as screen metrics are updated automatically
        Ok(1)
    }

    #[napi]
    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<i32> {
        self.enigo.mouse_move_to(x, y);
        std::thread::sleep(Duration::from_millis(self.mouse_delay));
        Ok(1)
    }

    #[napi]
    pub fn move_mouse_smooth(&mut self, x: i32, y: i32, speed: Option<f64>) -> Result<i32> {
        let speed = speed.unwrap_or(3.0);
        let (start_x, start_y) = self.enigo.mouse_location();
        let steps = ((((x - start_x).pow(2) + (y - start_y).pow(2)) as f64).sqrt() / speed) as i32;

        for i in 1..=steps {
            let ix = start_x + (x - start_x) * i / steps;
            let iy = start_y + (y - start_y) * i / steps;
            self.enigo.mouse_move_to(ix, iy);
            std::thread::sleep(Duration::from_millis(self.mouse_delay));
        }

        self.enigo.mouse_move_to(x, y);
        Ok(1)
    }

    #[napi]
    pub fn get_mouse_pos(&self, ctx: Env) -> Result<JsObject> {
        let (x, y) = self.enigo.mouse_location();
        let mut obj = ctx.create_object()?;
        obj.set("x", x)?;
        obj.set("y", y)?;
        Ok(obj)
    }

    #[napi]
    pub fn mouse_click(&mut self, button: Option<String>, double: Option<bool>) -> Result<i32> {
        let button = match button.as_deref() {
            Some("left") => MouseButton::Left,
            Some("right") => MouseButton::Right,
            Some("middle") => MouseButton::Middle,
            _ => MouseButton::Left,
        };
        self.enigo.mouse_click(button);
        if double.unwrap_or(false) {
            std::thread::sleep(Duration::from_millis(10));
            self.enigo.mouse_click(button);
        }
        std::thread::sleep(Duration::from_millis(self.mouse_delay));
        Ok(1)
    }

    #[napi]
    pub fn mouse_toggle(&mut self, down: Option<String>, button: Option<String>) -> Result<i32> {
        let button = match button.as_deref() {
            Some("left") => MouseButton::Left,
            Some("right") => MouseButton::Right,
            Some("middle") => MouseButton::Middle,
            _ => MouseButton::Left,
        };
        match down.as_deref() {
            Some("down") => self.enigo.mouse_down(button),
            Some("up") => self.enigo.mouse_up(button),
            _ => return Err(Error::from_reason("Invalid mouse button state specified.")),
        }
        std::thread::sleep(Duration::from_millis(self.mouse_delay));
        Ok(1)
    }

    #[napi]
    pub fn set_mouse_delay(&mut self, delay: f64) -> Result<i32> {
        self.mouse_delay = delay as u64;
        Ok(1)
    }

    #[napi]
    pub fn scroll_mouse(&mut self, x: i32, y: i32) -> Result<i32> {
        self.enigo.mouse_scroll_x(x);
        self.enigo.mouse_scroll_y(y);
        std::thread::sleep(Duration::from_millis(self.mouse_delay));
        Ok(1)
    }

    #[napi]
    pub fn key_tap(&mut self, key: String, modifiers: Option<Vec<String>>) -> Result<i32> {
        if let Some(ref mods) = modifiers {
            for mod_key in mods {
                self.enigo.key_down(Key::Layout(mod_key.chars().next().unwrap_or('\0')));
            }
        }
        self.enigo.key_click(Key::Layout(key.chars().next().unwrap_or('\0')));
        if let Some(ref mods) = modifiers {
            for mod_key in mods.iter().rev() {
                self.enigo.key_up(Key::Layout(mod_key.chars().next().unwrap_or('\0')));
            }
        }
        std::thread::sleep(Duration::from_millis(self.keyboard_delay));
        Ok(1)
    }

    #[napi]
    pub fn key_toggle(&mut self, key: String, down: Option<String>, modifiers: Option<Vec<String>>) -> Result<i32> {
        if let Some(ref mods) = modifiers {
            for mod_key in mods {
                self.enigo.key_down(Key::Layout(mod_key.chars().next().unwrap_or('\0')));
            }
        }
        match down.as_deref() {
            Some("down") => self.enigo.key_down(Key::Layout(key.chars().next().unwrap_or('\0'))),
            Some("up") => self.enigo.key_up(Key::Layout(key.chars().next().unwrap_or('\0'))),
            _ => return Err(Error::from_reason("Invalid key state specified.")),
        }
        if let Some(ref mods) = modifiers {
            for mod_key in mods.iter().rev() {
                self.enigo.key_up(Key::Layout(mod_key.chars().next().unwrap_or('\0')));
            }
        }
        std::thread::sleep(Duration::from_millis(self.keyboard_delay));
        Ok(1)
    }

    #[napi]
    pub fn unicode_tap(&mut self, value: String) -> Result<i32> {
        if let Some(c) = value.chars().next() {
            self.enigo.key_sequence(&c.to_string());
            std::thread::sleep(Duration::from_millis(self.keyboard_delay));
            Ok(1)
        } else {
            Err(Error::from_reason("Invalid unicode character"))
        }
    }

    #[napi]
    pub fn type_string(&mut self, string: String) -> Result<i32> {
        self.enigo.key_sequence(&string);
        std::thread::sleep(Duration::from_millis(self.keyboard_delay));
        Ok(1)
    }

    #[napi]
    pub fn type_string_delayed(&mut self, string: String, cpm: i32) -> Result<i32> {
        let delay = (60.0 / cpm as f64 * 1000.0) as u64;
        for c in string.chars() {
            self.enigo.key_sequence(&c.to_string());
            std::thread::sleep(Duration::from_millis(delay));
        }
        Ok(1)
    }

    #[napi]
    pub fn set_keyboard_delay(&mut self, delay: f64) -> Result<i32> {
        self.keyboard_delay = delay as u64;
        Ok(1)
    }

    #[napi]
    pub fn get_pixel_color(&self, x: i32, y: i32) -> Result<String> {
        let screen = Screen::from_point(x, y).unwrap();
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        let width = image.width() as usize;
        let height = image.height() as usize;

        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            return Err(Error::from_reason("Coordinates out of bounds"));
        }

        let index = (y as usize * width + x as usize) * 4;
        let r = buffer[index];
        let g = buffer[index + 1];
        let b = buffer[index + 2];

        Ok(format!("#{:02x}{:02x}{:02x}", r, g, b))
    }

    #[napi]
    pub fn get_screen_size(&self, ctx: Env) -> Result<JsObject> {
        let screen = Screen::all().unwrap()[0];
        let mut obj = ctx.create_object()?;
        obj.set("width", screen.display_info.width)?;
        obj.set("height", screen.display_info.height)?;
        Ok(obj)
    }

    #[napi]
    pub fn capture_screen(&self, ctx: Env, x: Option<i32>, y: Option<i32>, width: Option<i32>, height: Option<i32>) -> Result<JsObject> {
        let screen = Screen::all().unwrap()[0];
        let x = x.unwrap_or(0);
        let y = y.unwrap_or(0);
        let width = width.unwrap_or(screen.display_info.width as i32);
        let height = height.unwrap_or(screen.display_info.height as i32);

        let image = screen.capture_area(x, y, width as u32, height as u32).unwrap();

        let mut obj = ctx.create_object()?;
        obj.set("width", width)?;
        obj.set("height", height)?;
        obj.set("byteWidth", width * 4)?;
        obj.set("bitsPerPixel", 32)?;
        obj.set("bytesPerPixel", 4)?;
        obj.set("image", ctx.create_buffer_with_data(image.buffer().to_vec())?.into_raw())?;
        Ok(obj)
    }

    #[napi]
    pub fn get_color(&self, img: Object, x: i32, y: i32) -> Result<String> {
        let width: i32 = img.get("width")?.unwrap();
        let height: i32 = img.get("height")?.unwrap();
        let image_data: Vec<u8> = img.get("image")?.unwrap();

        if x < 0 || x >= width || y < 0 || y >= height {
            return Err(Error::from_reason("Coordinates out of bounds"));
        }

        let index = ((y * width + x) * 4) as usize;
        let r = image_data[index];
        let g = image_data[index + 1];
        let b = image_data[index + 2];

        Ok(format!("#{:02x}{:02x}{:02x}", r, g, b))
    }

    #[napi]
    pub fn get_x_display_name(&self) -> Result<String> {
        // This is a Linux-specific function, we'll return a placeholder for now
        Ok(String::from(""))
    }

    #[napi]
    pub fn set_x_display_name(&self, _name: String) -> Result<i32> {
        // This is a Linux-specific function, we'll return a success code for now
        Ok(1)
    }
}