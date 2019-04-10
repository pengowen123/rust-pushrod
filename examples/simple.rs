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

use std::cell::RefCell;

use piston_window::*;
use pushrod::core::callbacks::*;
use pushrod::core::main::*;
use pushrod::core::widget_store::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::image_widget::*;
use pushrod::widget::progress_widget::*;
use pushrod::widget::push_button_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::timer_widget::*;
use pushrod::widget::toggle_button_widget::*;
use pushrod::widget::widget::*;

pub struct SimpleWindow {
    pushrod: RefCell<Pushrod>,
}

pub struct SimpleWindowEventHandler {
    animated: bool,
}

impl PushrodCallbackEvents for SimpleWindowEventHandler {
    fn handle_event(&mut self, event: CallbackEvent, widget_store: &mut WidgetStore) {
//        eprintln!("Handle event: {:?}", event);

        match event {
            CallbackEvent::WidgetClicked { widget_id, button } => {
                if widget_id == 3 {
                    match button {
                        Button::Mouse(mouse_button) => {
                            if mouse_button == MouseButton::Left {
                                widget_store.get_widget_for_id(2).borrow_mut().set_color([
                                    (rand::random::<u8>() as f32 / 255.0),
                                    (rand::random::<u8>() as f32 / 255.0),
                                    (rand::random::<u8>() as f32 / 255.0),
                                    1.0,
                                ]);
                            }
                        }
                        _ => (),
                    }
                }
            }

            _ => (),
        }
    }
}

impl SimpleWindowEventHandler {
    fn new() -> Self {
        SimpleWindowEventHandler {
            animated: false,
        }
    }
}

impl SimpleWindow {
    fn new(prod: Pushrod) -> Self {
        Self {
            pushrod: RefCell::new(prod),
        }
    }

    fn add_hello_world(&mut self) {
        let mut text_widget = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Welcome to rust-pushrod!".to_string(),
            32,
            TextJustify::Left,
        );

        text_widget.set_origin(20, 20);
        text_widget.set_size(400, 40);
        text_widget.set_color([0.75, 0.75, 1.0, 1.0]);
        text_widget.set_text_color([0.75, 0.25, 1.0, 1.0]);

        self.pushrod.borrow_mut().add_widget(Box::new(text_widget));
    }

    fn add_base_widget(&mut self) {
        let mut base_widget = CanvasWidget::new();

        base_widget.set_origin(20, 80);
        base_widget.set_size(200, 200);
        base_widget.set_color([0.5, 0.5, 0.5, 1.0]);

        let base_widget_id = self.pushrod.borrow_mut().add_widget(Box::new(base_widget));

        let mut button1 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Random Color".to_string(),
            18,
            TextJustify::Center,
        );
        button1.set_origin(30, 236);
        button1.set_size(180, 32);
        button1.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button1.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(button1), base_widget_id);

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_origin(20, 290);
        button2.set_size(200, 32);
        button2.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button2.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod.borrow_mut().add_widget(Box::new(button2));
    }

    fn add_box_widgets(&mut self) {
        let mut box_widget = BoxWidget::new();

        box_widget.set_origin(250, 80);
        box_widget.set_size(200, 200);
        box_widget.set_color([0.0, 1.0, 0.0, 1.0]);
        box_widget.set_border([1.0, 0.0, 0.0, 1.0], 4);
        let box_widget_id = self.pushrod.borrow_mut().add_widget(Box::new(box_widget));

        let mut text_widget2 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Left".to_string(),
            24,
            TextJustify::Left,
        );
        text_widget2.set_origin(265, 100);
        text_widget2.set_size(170, 32);
        text_widget2.set_text_color([0.0, 0.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(text_widget2), box_widget_id);

        let mut text_widget3 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Center".to_string(),
            24,
            TextJustify::Center,
        );
        text_widget3.set_origin(265, 166);
        text_widget3.set_size(170, 32);
        text_widget3.set_text_color([0.0, 0.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(text_widget3), box_widget_id);

        let mut text_widget4 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Right".to_string(),
            24,
            TextJustify::Right,
        );
        text_widget4.set_origin(265, 230);
        text_widget4.set_size(170, 32);
        text_widget4.set_text_color([0.0, 0.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(text_widget4), box_widget_id);

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_origin(250, 290);
        button2.set_size(200, 32);
        button2.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button2.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod.borrow_mut().add_widget(Box::new(button2));

        let mut box_1 = BoxWidget::new();
        box_1.set_origin(480, 80);
        box_1.set_size(200, 200);
        box_1.set_color([0.5, 0.5, 1.0, 1.0]);
        box_1.set_border([0.0, 0.0, 1.0, 1.0], 2);
        let box_1_id = self.pushrod.borrow_mut().add_widget(Box::new(box_1));

        let mut inner_box_1 = BoxWidget::new();
        inner_box_1.set_origin(505, 105);
        inner_box_1.set_size(70, 60);
        inner_box_1.set_color([0.75, 0.75, 1.0, 1.0]);
        inner_box_1.set_border([1.0, 0.0, 1.0, 1.0], 1);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(inner_box_1), box_1_id);

        let mut inner_box_2 = BoxWidget::new();
        inner_box_2.set_origin(585, 105);
        inner_box_2.set_size(70, 60);
        inner_box_2.set_color([0.75, 0.25, 1.0, 1.0]);
        inner_box_2.set_border([1.0, 1.0, 0.0, 1.0], 1);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(inner_box_2), box_1_id);

        let mut inner_box_3 = BoxWidget::new();
        inner_box_3.set_origin(505, 190);
        inner_box_3.set_size(70, 60);
        inner_box_3.set_color([0.25, 0.50, 0.75, 1.0]);
        inner_box_3.set_border([1.0, 0.50, 1.0, 1.0], 1);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(inner_box_3), box_1_id);

        let mut inner_box_4 = BoxWidget::new();
        inner_box_4.set_origin(585, 190);
        inner_box_4.set_size(70, 60);
        inner_box_4.set_color([0.75, 0.50, 0.0, 1.0]);
        inner_box_4.set_border([0.50, 0.0, 0.25, 1.0], 1);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent(Box::new(inner_box_4), box_1_id);

        let mut button = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button.set_origin(480, 290);
        button.set_size(200, 32);
        button.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod.borrow_mut().add_widget(Box::new(button));
    }

    fn add_powered_by(&mut self) {
        let mut image_widget = ImageWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "rust-512x512.jpg".to_string(),
        );
        image_widget.set_origin(740, 540);
        image_widget.set_size(48, 48);
        self.pushrod.borrow_mut().add_widget(Box::new(image_widget));
    }

    fn add_progress(&mut self) {
        let mut progress_widget = ProgressWidget::new();

        progress_widget.set_origin(20, 360);
        progress_widget.set_size(300, 32);
        progress_widget.set_color([1.0, 1.0, 1.0, 1.0]);
        progress_widget.set_secondary_color([0.5, 0.5, 0.5, 1.0]);
        progress_widget.set_progress(50);
        self.pushrod
            .borrow_mut()
            .add_widget(Box::new(progress_widget));

        let mut button1 = ToggleButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Animate".to_string(),
            18,
            TextJustify::Center,
        );

        button1.set_origin(340, 360);
        button1.set_size(160, 32);
        button1.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button1.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod.borrow_mut().add_widget(Box::new(button1));

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Randomize".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_origin(520, 360);
        button2.set_size(160, 32);
        button2.set_text_color([0.0, 0.0, 0.0, 1.0]);
        button2.set_border([0.0, 0.0, 0.0, 1.0], 2);

        self.pushrod.borrow_mut().add_widget(Box::new(button2));
    }

    fn add_timer(&mut self) {
        let mut timer = TimerWidget::new();
        timer.set_timeout(10000);
        timer.set_enabled(true);
        self.pushrod.borrow_mut().add_widget(Box::new(timer));
    }

    fn build(&mut self) {
        self.add_hello_world();
        self.add_base_widget();
        self.add_box_widgets();
        self.add_powered_by();
        self.add_progress();
        self.add_timer();
    }

    fn get_pushrod(&mut self) -> &mut Pushrod {
        self.pushrod.get_mut()
    }

    pub fn run(&mut self) {
        let mut handler = SimpleWindowEventHandler::new();

        self.build();
        self.get_pushrod().run(&mut handler);
    }
}

fn main() {
    let window: PistonWindow = WindowSettings::new("Pushrod Window", [800, 600])
        .opengl(OpenGL::V3_2)
        .resizable(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    let mut app_window = SimpleWindow::new(Pushrod::new(window));

    app_window.run();
}
