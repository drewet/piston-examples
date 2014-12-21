#![feature(globs)]

extern crate current;
extern crate shader_version;
extern crate input;
extern crate event;
extern crate sdl2_window;
// extern crate glfw_window;

use current::{ Set };
use std::cell::RefCell;
use sdl2_window::Sdl2Window as Window;
// use glfw_window::GlfwWindow as Window;
use input::Button;
use input::keyboard::Key;
use event::{
    Events,
    FocusEvent,
    PressEvent,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    ReleaseEvent,
    RenderEvent,
    ResizeEvent,
    TextEvent,
    UpdateEvent,
    WindowSettings,
};
use event::window::{ CaptureCursor };

fn main() {
    let window = Window::new(
        shader_version::OpenGL::_3_2,
        WindowSettings {
            title: "piston-examples/user_input".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut capture_cursor = false;
    let ref window = RefCell::new(window);
    for e in Events::new(window) {
        e.press(|button| {
            match button {
                Button::Keyboard(key) => {
                    if key == Key::C {
                        println!("Turned capture cursor on");
                        capture_cursor = !capture_cursor;
                        window.set(CaptureCursor(capture_cursor));
                    }

                    println!("Pressed keyboard key '{}'", key);
                }, 
                Button::Mouse(button) => println!("Pressed mouse button '{}'", button),
            }
        });
        e.release(|button| {
            match button {
                Button::Keyboard(key) => println!("Released keyboard key '{}'", key),
                Button::Mouse(button) => println!("Released mouse button '{}'", button),
            }
        });
        e.mouse_cursor(|x, y| println!("Mouse moved '{} {}'", x, y));
        e.mouse_scroll(|dx, dy| println!("Scrolled mouse '{}, {}'", dx, dy));
        e.mouse_relative(|dx, dy| println!("Relative mouse moved '{} {}'", dx, dy));
        e.text(|text| println!("Typed '{}'", text));
        e.resize(|w, h| println!("Resized '{}, {}'", w, h));
        e.focus(|focused| {
            if focused { println!("Gained focus"); }
            else { println!("Lost focus"); }
        });
        e.render(|_| {});
        e.update(|_| {});
    }
}

