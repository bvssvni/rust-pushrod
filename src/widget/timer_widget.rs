// Timer Widget
// Timer-based widget that fires off a callback every time a certain time period is reached.
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

use piston_window::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::callbacks::CallbackEvent;
use crate::core::point::*;
use crate::widget::config::*;
use crate::widget::widget::*;

pub const CALLBACK_TIMER: u32 = 100;

/// This is the `TimerWidget`.  It contains no base widget, it only contains a start and end
/// time,
///
/// Example usage:
/// IN PROGRESS
pub struct TimerWidget {
    config: Configurable,
    enabled: bool,
    initiated: u64,
    timeout: u64,
    event: Option<CallbackEvent>,
}

/// Helper function that returns the current time in milliseconds since the `UNIX_EPOCH`.  This
/// function is the equivalent of a `System.currentTimeMillis()` in Java.
fn time_ms() -> u64 {
    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    (since_the_epoch.as_secs() * 1_000) + (since_the_epoch.subsec_nanos() / 1_000_000) as u64
}

/// Implementation of the constructor for the `TimerWidget`.  Timer widgets are not accessible
/// on the screen, so they have an origin of 0x0 and width of 0x0.
///
/// The timer provides a simple way to call a callback function after a certain amount of time
/// has passed.  Upon instantiation, the timer is enabled.
///
/// Here are a few limitations of the timer as it currently stands:
///
/// - Timer cannot be paused; it is enabled or disabled, and the timer resets when enabled.
/// - Timer is called when the screen refreshes, so slower FPS settings will affect the timer.
impl TimerWidget {
    /// Constructor, creates a new `TimerWidget` struct with an empty timeout function.
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            enabled: true,
            initiated: time_ms(),
            timeout: 0,
            event: None,
        }
    }

    // Called to check the time since initiation, and call the timeout function when a timer has
    // been triggered.
    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        let elapsed = time_ms() - self.initiated;

        if elapsed > self.timeout {
            self.initiated = time_ms();
            self.event = Some(CallbackEvent::TimerTriggered { widget_id: 0 });
        }
    }

    /// Enables or disables the timer.  When disabled, the timer will not initiate the callback
    /// function.  When re-enabled, the initiation time resets, so the timer will reset back to
    /// zero, effectively resetting the entire timer.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.initiated = time_ms();
    }

    /// Sets the timeout in milliseconds for this timer.  Will trigger a call to the function
    /// set in `on_timeout` when triggered, and will continue to call that function until this
    /// timer is disabled by using `self.set_enabled(false)`.
    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
}

/// Implementation of the `TimerWidget` object with the `Widget` traits implemented.
impl Widget for TimerWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    /// Timer is always invalidated, this way, the tick function is always called on each
    /// screen refresh.
    fn is_invalidated(&mut self) -> bool {
        true
    }

    /// Origin is always set to X/Y at points 0x0.
    fn get_origin(&mut self) -> Point {
        make_origin_point()
    }

    /// Size is always unsized, as timers are invisible.
    fn get_size(&mut self) -> crate::core::point::Size {
        make_unsized()
    }

    /// This function injects events, as a timeout event only occurs once.
    fn injects_events(&mut self) -> bool {
        true
    }

    /// Returns an injected event where appropriate.
    fn inject_event(&mut self) -> Option<CallbackEvent> {
        let return_obj = self.event.clone();

        self.event = None;
        return return_obj;
    }

    /// Does not draw anything - only calls the timer `tick()` function to increment the
    /// timer.
    fn draw(&mut self, _context: Context, _graphics: &mut G2d, _clip: &DrawState) {
        self.tick();
    }
}
