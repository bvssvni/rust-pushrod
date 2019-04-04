// Callback Store
// Callback Cache using fn() Enumerations for storing closures
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::core::point::Point;

use std::collections::HashMap;

use piston_window::*;

/// Index for mouse entered callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::SingleCallback``` callback.
pub const CALLBACK_MOUSE_ENTERED: u32 = 1;

/// Index for mouse exited callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::SingleCallback``` callback.
pub const CALLBACK_MOUSE_EXITED: u32 = 2;

/// Index for mouse scrolled callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::PointCallback``` callback.
pub const CALLBACK_MOUSE_SCROLLED: u32 = 3;

/// Index for mouse moved callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::PointCallback``` callback.
pub const CALLBACK_MOUSE_MOVED: u32 = 4;

/// Index for keyboard event callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::KeyCallback``` callback.
pub const CALLBACK_KEY_PRESSED: u32 = 5;

/// Index for window resized callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::SizeCallback``` callback.
pub const CALLBACK_WINDOW_RESIZED: u32 = 6;

/// Index for window focused callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::BoolCallback``` callback.
pub const CALLBACK_WINDOW_FOCUSED: u32 = 7;

/// Index for button click button down callback, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::ButtonCallback``` callback.
pub const CALLBACK_BUTTON_DOWN: u32 = 8;

/// Index for button click button release inside the same `Widget`, used by `Widget` internally.  Refers to a
/// ```CallbackTypes::ButtonCallback``` callback.
pub const CALLBACK_BUTTON_UP_INSIDE: u32 = 9;

/// Callback type that takes no input.
pub type BlankCallback = Box<Fn() -> ()>;

/// Callback type that accepts a widget ID.
pub type SingleCallback = Box<Fn(i32) -> ()>;

/// Callback type that accepts a boolean.
pub type BoolCallback = Box<Fn(i32, bool) -> ()>;

/// Callback type that accepts a widget ID and a `Point` on the screen as its input parameter.
pub type PointCallback = Box<Fn(i32, Point) -> ()>;

/// Callback type that accepts a widget ID and a `Size`.
pub type SizeCallback = Box<Fn(i32, crate::core::point::Size) -> ()>;

/// Callback type that accepts a widget ID, a key code, and its down/up button keypress state.
pub type KeyCallback = Box<Fn(i32, Key, ButtonState) -> ()>;

/// Callback type that accepts a widget ID and a button code.
pub type ButtonCallback = Box<Fn(i32, Button) -> ()>;

/// This is an enumerated type that is used to store numerous variations of callbacks that can
/// be used within the `Widget` system.  This is written such that the `CallbackTypes` enum
/// can be added to/extended as necessary.
pub enum CallbackTypes {
    /// Callback that calls a function without any data.
    BlankCallback { callback: BlankCallback },

    /// Callback that only supplies its widget ID.
    SingleCallback { callback: SingleCallback },

    /// Callback that supplies its widget ID and a boolean flag.
    BoolCallback { callback: BoolCallback },

    /// Callback that supplies its widget ID and a `Point` on the screen within the `Widget`.
    PointCallback { callback: PointCallback },

    /// Callback that supplies its widget ID and a `Size`.
    SizeCallback { callback: SizeCallback },

    /// Callback that supplies its widget ID, a key code, and button state.
    KeyCallback { callback: KeyCallback },

    /// Callback that supplies its widget ID and a button code.
    ButtonCallback { callback: ButtonCallback },
}

/// This is the `CallbackStore` that is used to store a list of `CallbackTypes` that are
/// triggered when an action occurs on a `Widget`.
pub struct CallbackStore {
    callbacks: HashMap<u32, CallbackTypes>,
}

/// The actual class implementation of the `CallbackStore`.  This is primarily stored within the
/// `Widget` class, and its usage looks something similar to the following code:
///
/// ```
/// # use pushrod::core::callbacks::*;
/// # use pushrod::core::point::*;
/// # fn main() {
///     let mut cs = CallbackStore::new();
///
///     cs.put(CALLBACK_MOUSE_MOVED,
///         CallbackTypes::PointCallback { callback: Box::new(|widget_id, point| {
///             eprintln!("Callback for widget {} resulted in point at {} x {}",
///                 widget_id, point.x, point.y);
///         })
///     });
///
///     // And, to call the callback to run it:
///
///     match cs.get(CALLBACK_MOUSE_MOVED) {
///         CallbackTypes::PointCallback { callback } =>
///             callback(12, Point { x: 16, y: 24 }),
///         _ => eprintln!("Unsupported callback for ID {}!", CALLBACK_MOUSE_MOVED),
///     }
/// # }
/// ```
///
/// This is an example of how it would be used in the `Widget` callbacks.  User-specified
/// callbacks will likely be much simpler than this.
impl CallbackStore {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }

    pub fn put(&mut self, id: u32, func: CallbackTypes) {
        self.callbacks.insert(id, func);
    }

    pub fn get(&mut self, id: u32) -> &CallbackTypes {
        if self.callbacks.contains_key(&id) {
            &self.callbacks[&id]
        } else {
            self.put(
                id,
                CallbackTypes::SingleCallback {
                    callback: Box::new(|_arg| {}),
                },
            );

            &self.callbacks[&id]
        }
    }
}
