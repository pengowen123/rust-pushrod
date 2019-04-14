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
use pushrod::core::point::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::image_widget::*;
use pushrod::widget::progress_widget::*;
use pushrod::widget::push_button_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::timer_widget::*;
use pushrod::widget::toggle_button_widget::*;
use pushrod::widget::widget::*;
use pushrod::widget::config::*;

pub struct SimpleWindow {
    pushrod: RefCell<Pushrod>,
}

pub struct SimpleWindowEventHandler {
    animated: bool,
    progress: u16,
}

impl PushrodCallbackEvents for SimpleWindowEventHandler {
    fn handle_event(&mut self, event: CallbackEvent, widget_store: &mut WidgetStore) {
        match event {
            CallbackEvent::MouseEntered { widget_id } => {
                // When a mouse enters a widget, the ID will get modified; modify the debug widget
                // with the ID that was specified.
                let widget_name = String::from(widget_store.get_name_for_widget_id(widget_id));
                let widget_point = widget_store
                    .get_widget_for_id(widget_id)
                    .borrow_mut()
                    .config()
                    .get_point(CONFIG_ORIGIN);
                let widget_size = widget_store
                    .get_widget_for_id(widget_id)
                    .borrow_mut()
                    .config()
                    .get_size(CONFIG_BODY_SIZE);

                widget_store
                    .get_widget_for_name("DebugText1")
                    .borrow_mut()
                    .set_config(CONFIG_DISPLAY_TEXT,
                         Config::Text(format!("Current Widget: {} ({})", widget_id, widget_name)).clone());

                widget_store
                    .get_widget_for_name("DebugText2")
                    .borrow_mut()
                    .set_config(CONFIG_DISPLAY_TEXT,
                        Config::Text(format!(
                            "Dimensions: x={} y={} w={} h={}",
                            widget_point.x, widget_point.y, widget_size.w, widget_size.h
                        )).clone()
                    );
            }

            CallbackEvent::WidgetClicked { widget_id, button } => {
                match widget_store.get_name_for_widget_id(widget_id) {
                    "RandomColorButton1" => match button {
                        Button::Mouse(mouse_button) => {
                            if mouse_button == MouseButton::Left {
                                widget_store
                                    .get_widget_for_name("BaseWidget1")
                                    .borrow_mut()
                                    .set_config(CONFIG_MAIN_COLOR, Config::Color([
                                        (rand::random::<u8>() as f32 / 255.0),
                                        (rand::random::<u8>() as f32 / 255.0),
                                        (rand::random::<u8>() as f32 / 255.0),
                                        1.0,
                                    ]));
                            }
                        }
                        _ => (),
                    },

                    "RandomColorButton2" => match button {
                        Button::Mouse(mouse_button) => {
                            if mouse_button == MouseButton::Left {
                                widget_store
                                    .get_widget_for_name("ProgressWidget")
                                    .borrow_mut()
                                    .set_config(CONFIG_SECONDARY_COLOR, Config::Color([
                                        (rand::random::<u8>() as f32 / 255.0),
                                        (rand::random::<u8>() as f32 / 255.0),
                                        (rand::random::<u8>() as f32 / 255.0),
                                        1.0,
                                    ]));
                            }
                        }
                        _ => (),
                    },

                    x => eprintln!("Widget clicked: {}", x),
                }
            }

            CallbackEvent::WidgetSelected {
                widget_id,
                button: _,
                selected,
            } => match widget_store.get_name_for_widget_id(widget_id) {
                "AnimateButton1" => {
                    self.animated = selected;
                }
                _ => (),
            },

            CallbackEvent::TimerTriggered { widget_id: _ } => {
                if self.animated {
                    self.progress += 1;

                    if self.progress > 100 {
                        self.progress = 0;
                    }

                    widget_store
                        .get_widget_for_name("ProgressWidget")
                        .borrow_mut()
                        .set_config(CONFIG_PROGRESS, Config::Numeric(self.progress as u64));
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
            progress: 50,
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

        text_widget.set_point(CONFIG_ORIGIN, 20, 20);
        text_widget.set_size(CONFIG_BODY_SIZE, 400, 40);
        text_widget.set_color(CONFIG_MAIN_COLOR, [0.75, 0.75, 1.0, 1.0]);
        text_widget.set_color(CONFIG_TEXT_COLOR, [0.75, 0.25, 1.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("TextWidget", Box::new(text_widget));
    }

    fn add_base_widget(&mut self) {
        let mut base_widget = CanvasWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 80);
        base_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
        base_widget.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 0.5, 1.0]);

        let base_widget_id = self
            .pushrod
            .borrow_mut()
            .add_widget("BaseWidget1", Box::new(base_widget));

        let mut button1 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Random Color".to_string(),
            18,
            TextJustify::Center,
        );
        button1.set_point(CONFIG_ORIGIN, 30, 236);
        button1.set_size(CONFIG_BODY_SIZE, 180, 32);
        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod.borrow_mut().add_widget_to_parent(
            "RandomColorButton1",
            Box::new(button1),
            base_widget_id,
        );

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_point(CONFIG_ORIGIN, 20, 290);
        button2.set_size(CONFIG_BODY_SIZE, 200, 32);
        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("HideButton1", Box::new(button2));
    }

    fn add_box_widgets(&mut self) {
        let mut box_widget = BoxWidget::new();

        box_widget.set_point(CONFIG_ORIGIN, 250, 80);
        box_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_widget.set_color(CONFIG_MAIN_COLOR, [0.0, 1.0, 0.0, 1.0]);
        box_widget.set_numeric(CONFIG_BORDER_WIDTH, 4);
        box_widget.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 0.0, 1.0]);
        let box_widget_id = self
            .pushrod
            .borrow_mut()
            .add_widget("BoxWidget1", Box::new(box_widget));

        let mut text_widget2 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Left".to_string(),
            24,
            TextJustify::Left,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 265, 100);
        text_widget2.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "LeftJustifiedText",
            Box::new(text_widget2),
            box_widget_id,
        );

        let mut text_widget3 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Center".to_string(),
            24,
            TextJustify::Center,
        );
        text_widget3.set_point(CONFIG_ORIGIN, 265, 166);
        text_widget3.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "CenterJustifiedText",
            Box::new(text_widget3),
            box_widget_id,
        );

        let mut text_widget4 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Right".to_string(),
            24,
            TextJustify::Right,
        );
        text_widget4.set_point(CONFIG_ORIGIN, 265, 230);
        text_widget4.set_size(CONFIG_BODY_SIZE, 170, 32);
        text_widget4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod.borrow_mut().add_widget_to_parent(
            "RightJustifiedText",
            Box::new(text_widget4),
            box_widget_id,
        );

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_point(CONFIG_ORIGIN, 250, 290);
        button2.set_size(CONFIG_BODY_SIZE, 200, 32);
        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("HideButton2", Box::new(button2));

        let mut box_1 = BoxWidget::new();
        box_1.set_point(CONFIG_ORIGIN, 480, 80);
        box_1.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_1.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 1.0, 1.0]);
        box_1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        box_1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 1.0, 1.0]);
        let box_1_id = self
            .pushrod
            .borrow_mut()
            .add_widget("Box1", Box::new(box_1));

        let mut inner_box_1 = BoxWidget::new();
        inner_box_1.set_point(CONFIG_ORIGIN, 505, 105);
        inner_box_1.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_1.set_color(CONFIG_MAIN_COLOR, [0.75, 0.75, 1.0, 1.0]);
        inner_box_1.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_1.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("Box2", Box::new(inner_box_1), box_1_id);

        let mut inner_box_2 = BoxWidget::new();
        inner_box_2.set_point(CONFIG_ORIGIN, 585, 105);
        inner_box_2.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_2.set_color(CONFIG_MAIN_COLOR, [0.75, 0.25, 1.0, 1.0]);
        inner_box_2.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_2.set_color(CONFIG_BORDER_COLOR, [1.0, 1.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("Box3", Box::new(inner_box_2), box_1_id);

        let mut inner_box_3 = BoxWidget::new();
        inner_box_3.set_point(CONFIG_ORIGIN, 505, 190);
        inner_box_3.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_3.set_color(CONFIG_MAIN_COLOR, [0.25, 0.50, 0.75, 1.0]);
        inner_box_3.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_3.set_color(CONFIG_BORDER_COLOR, [1.0, 0.50, 1.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("Box4", Box::new(inner_box_3), box_1_id);

        let mut inner_box_4 = BoxWidget::new();
        inner_box_4.set_point(CONFIG_ORIGIN, 585, 190);
        inner_box_4.set_size(CONFIG_BODY_SIZE, 70, 60);
        inner_box_4.set_color(CONFIG_MAIN_COLOR, [0.75, 0.50, 0.0, 1.0]);
        inner_box_4.set_numeric(CONFIG_BORDER_WIDTH, 1);
        inner_box_4.set_color(CONFIG_BORDER_COLOR, [0.50, 0.0, 0.25, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget_to_parent("Box5", Box::new(inner_box_4), box_1_id);

        let mut button = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Hide".to_string(),
            18,
            TextJustify::Center,
        );

        button.set_point(CONFIG_ORIGIN, 480, 290);
        button.set_size(CONFIG_BODY_SIZE, 200, 32);
        button.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("HideButton3", Box::new(button));
    }

    fn add_powered_by(&mut self) {
        let mut image_widget = ImageWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "rust-512x512.jpg".to_string(),
        );
        image_widget.set_point(CONFIG_ORIGIN, 740, 540);
        image_widget.set_size(CONFIG_BODY_SIZE, 48, 48);
        self.pushrod
            .borrow_mut()
            .add_widget("RustImage", Box::new(image_widget));
    }

    fn add_progress(&mut self) {
        let mut progress_widget = ProgressWidget::new();

        progress_widget.set_point(CONFIG_ORIGIN, 20, 360);
        progress_widget.set_size(CONFIG_BODY_SIZE, 300, 32);
        progress_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
        progress_widget.set_color(CONFIG_SECONDARY_COLOR, [0.5, 0.5, 0.5, 1.0]);
        progress_widget.set_numeric(CONFIG_PROGRESS, 50);
        self.pushrod
            .borrow_mut()
            .add_widget("ProgressWidget", Box::new(progress_widget));

        let mut button1 = ToggleButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Animate".to_string(),
            18,
            TextJustify::Center,
            false,
        );

        button1.set_point(CONFIG_ORIGIN, 340, 360);
        button1.set_size(CONFIG_BODY_SIZE, 160, 32);
        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("AnimateButton1", Box::new(button1));

        let mut button2 = PushButtonWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Randomize".to_string(),
            18,
            TextJustify::Center,
        );

        button2.set_point(CONFIG_ORIGIN, 520, 360);
        button2.set_size(CONFIG_BODY_SIZE, 160, 32);
        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);

        self.pushrod
            .borrow_mut()
            .add_widget("RandomColorButton2", Box::new(button2));
    }

    fn add_timer(&mut self) {
        let mut timer = TimerWidget::new();

        timer.set_numeric(CONFIG_TIMER_TIMEOUT, 100);
        timer.set_toggle(CONFIG_TIMER_ENABLED, true);
        self.pushrod
            .borrow_mut()
            .add_widget("TimerWidget1", Box::new(timer));
    }

    fn add_debugging(&mut self) {
        let mut text_widget1 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "Current Widget: 0".to_string(),
            20,
            TextJustify::Left,
        );
        text_widget1.set_point(CONFIG_ORIGIN, 20, 530);
        text_widget1.set_size(CONFIG_BODY_SIZE, 400, 28);
        text_widget1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("DebugText1", Box::new(text_widget1));

        let mut text_widget2 = TextWidget::new(
            self.pushrod.borrow_mut().get_factory(),
            "OpenSans-Regular.ttf".to_string(),
            "".to_string(),
            20,
            TextJustify::Left,
        );
        text_widget2.set_point(CONFIG_ORIGIN, 20, 560);
        text_widget2.set_size(CONFIG_BODY_SIZE, 400, 28);
        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
        self.pushrod
            .borrow_mut()
            .add_widget("DebugText2", Box::new(text_widget2));
    }

    fn build(&mut self) {
        self.add_hello_world();
        self.add_base_widget();
        self.add_box_widgets();
        self.add_powered_by();
        self.add_progress();
        self.add_timer();
        self.add_debugging();
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
