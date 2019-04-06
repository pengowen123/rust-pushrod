// Simple Windowed Example
// Super simplistic test to show off the use of the library in its current state
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

extern crate pushrod;

use piston_window::*;
use pushrod::core::main::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::image_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::timer_widget::*;
use pushrod::widget::widget::*;

fn main() {
    let window: PistonWindow = WindowSettings::new("Pushrod Window", [800, 600])
        .opengl(OpenGL::V3_2)
        .resizable(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    let mut prod: Pushrod = Pushrod::new(window);

    let mut text_widget = TextWidget::new(
        prod.get_factory(),
        "OpenSans-Regular.ttf".to_string(),
        "Welcome to rust-pushrod!".to_string(),
        32,
    );
    text_widget.set_origin(14, 8);
    text_widget.set_size(400, 48);
    text_widget.set_color([0.75, 0.75, 1.0, 1.0]);
    text_widget.set_text_color([0.75, 0.25, 1.0, 1.0]);
    prod.widget_store.add_widget(Box::new(text_widget));

    let mut base_widget = CanvasWidget::new();
    base_widget.set_origin(50, 80);
    base_widget.set_size(200, 200);
    base_widget.set_color([0.5, 0.5, 0.5, 1.0]);
    base_widget.on_mouse_entered(Box::new(|widget_id| {
        eprintln!("Mouse entered widget {}", widget_id);
    }));
    base_widget.on_mouse_moved(Box::new(|_, point| {
        eprintln!("Relative mouse move: {:?}", point);
    }));
    base_widget.on_button_down(Box::new(|_, button| {
        eprintln!("Mouse button down: {:?}", button);
    }));
    base_widget.on_button_up_inside(Box::new(|_, button| {
        eprintln!("Mouse button released (inside same widget): {:?}", button);
    }));
    base_widget.on_button_up_outside(Box::new(|_, button| {
        eprintln!("Mouse button release (outside widget): {:?}", button);
    }));
    prod.widget_store.add_widget(Box::new(base_widget));

    let mut box_widget = BoxWidget::new();
    box_widget.set_origin(275, 80);
    box_widget.set_size(200, 200);
    box_widget.set_color([0.0, 1.0, 0.0, 1.0]);
    box_widget.set_border([1.0, 0.0, 0.0, 1.0], 4);
    box_widget.on_key_pressed(Box::new(|_, key, state| {
        eprintln!("Key {:?}; State {:?}", key, state);
    }));
    prod.widget_store.add_widget(Box::new(box_widget));

    let mut box_1 = BoxWidget::new();
    box_1.set_origin(500, 80);
    box_1.set_size(200, 200);
    box_1.set_color([0.5, 0.5, 1.0, 1.0]);
    box_1.set_border([0.0, 0.0, 1.0, 1.0], 2);
    let box_1_id = prod.widget_store.add_widget(Box::new(box_1));

    let mut inner_box_1 = BoxWidget::new();
    inner_box_1.set_origin(525, 105);
    inner_box_1.set_size(70, 60);
    inner_box_1.set_color([0.75, 0.75, 1.0, 1.0]);
    inner_box_1.set_border([1.0, 0.0, 1.0, 1.0], 1);
    prod.widget_store
        .add_widget_to_parent(Box::new(inner_box_1), box_1_id);

    let mut inner_box_2 = BoxWidget::new();
    inner_box_2.set_origin(605, 105);
    inner_box_2.set_size(70, 60);
    inner_box_2.set_color([0.75, 0.25, 1.0, 1.0]);
    inner_box_2.set_border([1.0, 1.0, 0.0, 1.0], 1);
    prod.widget_store
        .add_widget_to_parent(Box::new(inner_box_2), box_1_id);

    let mut inner_box_3 = BoxWidget::new();
    inner_box_3.set_origin(525, 190);
    inner_box_3.set_size(70, 60);
    inner_box_3.set_color([0.25, 0.50, 0.75, 1.0]);
    inner_box_3.set_border([1.0, 0.50, 1.0, 1.0], 1);
    prod.widget_store
        .add_widget_to_parent(Box::new(inner_box_3), box_1_id);

    let mut inner_box_4 = BoxWidget::new();
    inner_box_4.set_origin(605, 190);
    inner_box_4.set_size(70, 60);
    inner_box_4.set_color([0.75, 0.50, 0.0, 1.0]);
    inner_box_4.set_border([0.50, 0.0, 0.25, 1.0], 1);
    prod.widget_store
        .add_widget_to_parent(Box::new(inner_box_4), box_1_id);

    let mut image_widget = ImageWidget::new(prod.get_factory(), "rust-512x512.jpg".to_string());
    image_widget.set_origin(50, 300);
    image_widget.set_size(125, 125);
    prod.widget_store.add_widget(Box::new(image_widget));

    let mut timer = TimerWidget::new();
    timer.set_timeout(1000);
    timer.set_enabled(true);
    timer.on_timeout(Box::new(|| eprintln!("Timer.")));
    prod.widget_store.add_widget(Box::new(timer));

    //    prod.add_event_listener_for_window(Box::new(ExampleListener::new()));

    // Runs the main event loop
    prod.run();
}
