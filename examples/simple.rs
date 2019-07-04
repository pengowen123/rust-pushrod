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

extern crate glfw_window;
extern crate pushrod;

use std::cell::RefCell;

use glfw_window::GlfwWindow;
use opengl_graphics::OpenGL;
use piston::input::*;
use piston::window::*;

use pushrod::core::callbacks::*;
use pushrod::core::horizontal_layout_manager::*;
use pushrod::core::main::*;
use pushrod::core::point::{make_origin_point, make_point_i32};
use pushrod::core::widget_store::*;
use pushrod::widget::box_widget::*;
use pushrod::widget::checkbox_widget::*;
use pushrod::widget::config::*;
use pushrod::widget::image_button_widget::*;
use pushrod::widget::progress_widget::*;
use pushrod::widget::push_button_widget::*;
use pushrod::widget::radio_button_widget::*;
use pushrod::widget::text_widget::*;
use pushrod::widget::timer_widget::*;
use pushrod::widget::toggle_button_widget::*;
use pushrod::widget::widget::*;

pub struct SimpleWindow {
    pushrod: RefCell<Pushrod>,
}

pub struct SimpleWindowEventHandler {
    animated: bool,
    progress: u16,
    red_value: i16,
    red_direction: i16,
}

impl PushrodCallbackEvents for SimpleWindowEventHandler {
    fn widget_clicked(&mut self, widget_id: i32, button: Button, widget_store: &mut WidgetStore) {
        //        match button {
        //            Button::Mouse(mouse_button) => {
        //                if mouse_button != MouseButton::Left {
        //                    return;
        //                }
        //            }
        //            _ => (),
        //        }
        //
        //        match widget_store.get_name_for_widget_id(widget_id) {
        //            "HideShowMainContainerWidgetButton" => {
        //                let main_container_widget_id =
        //                    widget_store.get_widget_id_for_name("MainContainerWidget");
        //
        //                let state = widget_store
        //                    .get_widget_for_name("MainContainerWidget")
        //                    .borrow_mut()
        //                    .config()
        //                    .get_toggle(CONFIG_WIDGET_HIDDEN);
        //
        //                let button_text = if state == true {
        //                    String::from("Hide")
        //                } else {
        //                    String::from("Show")
        //                };
        //
        //                widget_store
        //                    .get_widget_for_name("HideShowMainContainerWidgetButton")
        //                    .borrow_mut()
        //                    .set_text(CONFIG_DISPLAY_TEXT, button_text);
        //
        //                widget_store.set_hidden(main_container_widget_id, !state);
        //                widget_store.invalidate_all_widgets();
        //            }
        //
        //            "RandomColorButton1" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        widget_store
        //                            .get_widget_for_name("BaseWidget1")
        //                            .borrow_mut()
        //                            .set_config(
        //                                CONFIG_MAIN_COLOR,
        //                                Config::Color([
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    1.0,
        //                                ]),
        //                            );
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            "RandomColorButton2" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        widget_store
        //                            .get_widget_for_name("ProgressWidget")
        //                            .borrow_mut()
        //                            .set_config(
        //                                CONFIG_SECONDARY_COLOR,
        //                                Config::Color([
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    (rand::random::<u8>() as f32 / 255.0),
        //                                    1.0,
        //                                ]),
        //                            );
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            "HideButton1" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        let state = widget_store
        //                            .get_widget_for_name("BaseWidget1")
        //                            .borrow_mut()
        //                            .config()
        //                            .get_toggle(CONFIG_WIDGET_HIDDEN);
        //                        let button_text = if state == true {
        //                            String::from("Hide")
        //                        } else {
        //                            String::from("Show")
        //                        };
        //
        //                        widget_store
        //                            .get_widget_for_name("HideButton1")
        //                            .borrow_mut()
        //                            .set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));
        //
        //                        widget_store
        //                            .get_widget_for_name("RandomColorButton1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("BaseWidget1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //
        //                        widget_store.invalidate_all_widgets();
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            "HideButton2" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        let state = widget_store
        //                            .get_widget_for_name("BoxWidget1")
        //                            .borrow_mut()
        //                            .config()
        //                            .get_toggle(CONFIG_WIDGET_HIDDEN);
        //                        let button_text = if state == true {
        //                            String::from("Hide")
        //                        } else {
        //                            String::from("Show")
        //                        };
        //
        //                        widget_store
        //                            .get_widget_for_name("HideButton2")
        //                            .borrow_mut()
        //                            .set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));
        //
        //                        widget_store
        //                            .get_widget_for_name("BoxWidget1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("LeftJustifiedText")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("CenterJustifiedText")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("RightJustifiedText")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //
        //                        widget_store.invalidate_all_widgets();
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            "HideButton3" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        let state = widget_store
        //                            .get_widget_for_name("Box1")
        //                            .borrow_mut()
        //                            .config()
        //                            .get_toggle(CONFIG_WIDGET_HIDDEN);
        //                        let button_text = if state == true {
        //                            String::from("Hide")
        //                        } else {
        //                            String::from("Show")
        //                        };
        //
        //                        widget_store
        //                            .get_widget_for_name("HideButton3")
        //                            .borrow_mut()
        //                            .set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));
        //
        //                        widget_store
        //                            .get_widget_for_name("Box1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("Box2")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("Box3")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("Box4")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //                        widget_store
        //                            .get_widget_for_name("Box5")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_HIDDEN, !state);
        //
        //                        widget_store.invalidate_all_widgets();
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            "DisableButton1" => match button {
        //                Button::Mouse(mouse_button) => {
        //                    if mouse_button == MouseButton::Left {
        //                        let state = widget_store
        //                            .get_widget_for_name("BaseWidget1")
        //                            .borrow_mut()
        //                            .config()
        //                            .get_toggle(CONFIG_WIDGET_DISABLED);
        //                        let button_text = if state == true {
        //                            String::from("Disable")
        //                        } else {
        //                            String::from("Enable")
        //                        };
        //
        //                        widget_store
        //                            .get_widget_for_name("DisableButton1")
        //                            .borrow_mut()
        //                            .set_config(CONFIG_DISPLAY_TEXT, Config::Text(button_text));
        //
        //                        widget_store
        //                            .get_widget_for_name("RandomColorButton1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_DISABLED, !state);
        //                        widget_store
        //                            .get_widget_for_name("BaseWidget1")
        //                            .borrow_mut()
        //                            .set_toggle(CONFIG_WIDGET_DISABLED, !state);
        //
        //                        widget_store.invalidate_all_widgets();
        //                    }
        //                }
        //                _ => (),
        //            },
        //
        //            _ => (),
        //        }
    }

    fn timer_triggered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {
        match widget_store.get_name_for_widget_id(widget_id) {
            "HelloWorldTimer" => {
                if self.red_direction == 1 {
                    if self.red_value == 255 {
                        self.red_direction = -1;
                    }
                } else {
                    if self.red_value == 0 {
                        self.red_direction = 1;
                    }
                }

                self.red_value += self.red_direction;

                widget_store
                    .get_widget_for_name("TextWidget")
                    .borrow_mut()
                    .set_color(
                        CONFIG_TEXT_COLOR,
                        [(self.red_value as f32 / 255.0), 0.0, 0.0, 1.0],
                    );
            }

            _ => {}
        };
        //        if widget_store.get_name_for_widget_id(widget_id) == "TimerWidget1" {
        //            if self.animated {
        //                self.progress += 1;
        //
        //                if self.progress > 100 {
        //                    self.progress = 0;
        //                }
        //
        //                widget_store
        //                    .get_widget_for_name("ProgressWidget")
        //                    .borrow_mut()
        //                    .set_config(CONFIG_PROGRESS, Config::Numeric(self.progress as u64));
        //
        //                widget_store
        //                    .get_widget_for_name("ProgressText1")
        //                    .borrow_mut()
        //                    .set_text(CONFIG_DISPLAY_TEXT, format!("{} %", self.progress));
        //            }
        //        }
    }

    fn mouse_entered(&mut self, widget_id: i32, widget_store: &mut WidgetStore) {
        //        // When a mouse enters a widget, the ID will get modified; modify the debug widget
        //        // with the ID that was specified.
        //        let widget_name = String::from(widget_store.get_name_for_widget_id(widget_id));
        //        let widget_point = widget_store
        //            .get_widget_for_id(widget_id)
        //            .borrow_mut()
        //            .config()
        //            .get_point(CONFIG_ORIGIN);
        //        let widget_size = widget_store
        //            .get_widget_for_id(widget_id)
        //            .borrow_mut()
        //            .config()
        //            .get_size(CONFIG_BODY_SIZE);
        //
        //        widget_store
        //            .get_widget_for_name("DebugText1")
        //            .borrow_mut()
        //            .set_config(
        //                CONFIG_DISPLAY_TEXT,
        //                Config::Text(format!("Current Widget: {} ({})", widget_id, widget_name)).clone(),
        //            );
        //
        //        widget_store
        //            .get_widget_for_name("DebugText2")
        //            .borrow_mut()
        //            .set_config(
        //                CONFIG_DISPLAY_TEXT,
        //                Config::Text(format!(
        //                    "Dimensions: x={} y={} w={} h={}",
        //                    widget_point.x, widget_point.y, widget_size.w, widget_size.h
        //                ))
        //                .clone(),
        //            );
    }

    fn widget_selected(
        &mut self,
        widget_id: i32,
        _button: Button,
        selected: bool,
        widget_store: &mut WidgetStore,
    ) {
        //        match widget_store.get_name_for_widget_id(widget_id) {
        //            "AnimateButton1" => {
        //                self.animated = selected;
        //            }
        //
        //            "DebugCheck1" => {
        //                widget_store
        //                    .get_widget_for_name("DebugText1")
        //                    .borrow_mut()
        //                    .set_toggle(CONFIG_WIDGET_HIDDEN, !selected);
        //                widget_store
        //                    .get_widget_for_name("DebugText2")
        //                    .borrow_mut()
        //                    .set_toggle(CONFIG_WIDGET_HIDDEN, !selected);
        //            }
        //
        //            "Radio1" => {
        //                widget_store
        //                    .get_widget_for_name("TimerWidget1")
        //                    .borrow_mut()
        //                    .config()
        //                    .set_numeric(CONFIG_TIMER_TIMEOUT, 100);
        //            }
        //
        //            "Radio2" => {
        //                widget_store
        //                    .get_widget_for_name("TimerWidget1")
        //                    .borrow_mut()
        //                    .config()
        //                    .set_numeric(CONFIG_TIMER_TIMEOUT, 300);
        //            }
        //
        //            "Radio3" => {
        //                widget_store
        //                    .get_widget_for_name("TimerWidget1")
        //                    .borrow_mut()
        //                    .config()
        //                    .set_numeric(CONFIG_TIMER_TIMEOUT, 500);
        //            }
        //
        //            _ => (),
        //        }
    }
}

impl SimpleWindowEventHandler {
    fn new() -> Self {
        SimpleWindowEventHandler {
            animated: true,
            progress: 50,
            red_value: 0,
            red_direction: 1,
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
            "assets/OpenSans-Regular.ttf".to_string(),
            "Welcome to Pushrod".to_string(),
            36,
            TextJustify::Left,
        );

        text_widget.set_point(CONFIG_ORIGIN, 20, 10);
        text_widget.set_size(CONFIG_BODY_SIZE, 400, 50);
        text_widget.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
        text_widget.set_color(CONFIG_TEXT_COLOR, [0.75, 0.25, 1.0, 1.0]);

        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "TextWidget",
            Box::new(text_widget),
        );

        let mut timer = TimerWidget::new();

        timer.set_numeric(CONFIG_TIMER_TIMEOUT, 10);
        timer.set_toggle(CONFIG_TIMER_ENABLED, true);
        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HelloWorldTimer",
            Box::new(timer),
        );
    }

    fn add_horizontal_layout(&mut self) {
        let mut base_widget: CanvasWidget = CanvasWidget::new();

        base_widget.set_point(CONFIG_ORIGIN, 20, 70);
        base_widget.set_size(CONFIG_BODY_SIZE, 760, 200);
        base_widget.set_color(CONFIG_MAIN_COLOR, [0.25, 0.50, 0.75, 1.0]);
        //        base_widget.set_numeric(CONFIG_PADDING_TOP, 4);
        //        base_widget.set_numeric(CONFIG_PADDING_BOTTOM, 4);
        //        base_widget.set_numeric(CONFIG_PADDING_RIGHT, 4);
        //        base_widget.set_numeric(CONFIG_PADDING_LEFT, 4);

        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
            "MainContainerWidget",
            "HorizontalManagerWidget1",
            Box::new(base_widget),
        );

        let base_layout_id = self
            .pushrod
            .borrow_mut()
            .add_layout_manager(Box::new(HorizontalLayoutManager::new(base_widget_id)));

        let mut box_widget = BoxWidget::new();

        box_widget.set_point(CONFIG_ORIGIN, 250, 80);
        box_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
        box_widget.set_color(CONFIG_MAIN_COLOR, [0.0, 1.0, 0.0, 1.0]);
        box_widget.set_numeric(CONFIG_BORDER_WIDTH, 4);
        box_widget.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 0.0, 1.0]);
        let box_widget_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
            "BoxWidget1",
            Box::new(box_widget),
            base_layout_id,
            make_origin_point(),
        );
    }

    //    fn add_base_widget(&mut self) {
    //        let mut base_widget = CanvasWidget::new();
    //
    //        base_widget.set_point(CONFIG_ORIGIN, 20, 80);
    //        base_widget.set_size(CONFIG_BODY_SIZE, 200, 200);
    //        base_widget.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 0.5, 1.0]);
    //
    //        let base_widget_id = self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "BaseWidget1",
    //            Box::new(base_widget),
    //        );
    //
    //        let mut button1 = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Random Color".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //        button1.set_point(CONFIG_ORIGIN, 30, 236);
    //        button1.set_size(CONFIG_BODY_SIZE, 180, 32);
    //        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent(
    //            "RandomColorButton1",
    //            Box::new(button1),
    //            base_widget_id,
    //        );
    //
    //        let mut button2 = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Hide".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button2.set_point(CONFIG_ORIGIN, 20, 290);
    //        button2.set_size(CONFIG_BODY_SIZE, 95, 32);
    //        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "HideButton1",
    //            Box::new(button2),
    //        );
    //
    //        let mut button3 = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Disable".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button3.set_point(CONFIG_ORIGIN, 125, 290);
    //        button3.set_size(CONFIG_BODY_SIZE, 95, 32);
    //        button3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button3.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button3.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "DisableButton1",
    //            Box::new(button3),
    //        );
    //    }
    //
    //    fn add_box_widgets(&mut self) {
    //        let mut base_widget: CanvasWidget = CanvasWidget::new();
    //
    //        eprintln!("Base Layout Manager ID: {}", base_layout_id);
    //
    //        let mut text_widget2 = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Left".to_string(),
    //            24,
    //            TextJustify::Left,
    //        );
    //        text_widget2.set_point(CONFIG_ORIGIN, 265, 100);
    //        text_widget2.set_size(CONFIG_BODY_SIZE, 170, 32);
    //        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        text_widget2.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent(
    //            "LeftJustifiedText",
    //            Box::new(text_widget2),
    //            box_widget_id,
    //        );
    //
    //        let mut text_widget3 = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Center".to_string(),
    //            24,
    //            TextJustify::Center,
    //        );
    //        text_widget3.set_point(CONFIG_ORIGIN, 265, 166);
    //        text_widget3.set_size(CONFIG_BODY_SIZE, 170, 32);
    //        text_widget3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        text_widget3.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent(
    //            "CenterJustifiedText",
    //            Box::new(text_widget3),
    //            box_widget_id,
    //        );
    //
    //        let mut text_widget4 = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Right".to_string(),
    //            24,
    //            TextJustify::Right,
    //        );
    //        text_widget4.set_point(CONFIG_ORIGIN, 265, 230);
    //        text_widget4.set_size(CONFIG_BODY_SIZE, 170, 32);
    //        text_widget4.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        text_widget4.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 0.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent(
    //            "RightJustifiedText",
    //            Box::new(text_widget4),
    //            box_widget_id,
    //        );
    //
    //        let mut button2 = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Hide".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button2.set_point(CONFIG_ORIGIN, 250, 290);
    //        button2.set_size(CONFIG_BODY_SIZE, 200, 32);
    //        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "HideButton2",
    //            Box::new(button2),
    //        );
    //
    //        let mut box_1 = BoxWidget::new();
    //        box_1.set_point(CONFIG_ORIGIN, 480, 80);
    //        box_1.set_size(CONFIG_BODY_SIZE, 200, 200);
    //        box_1.set_color(CONFIG_MAIN_COLOR, [0.5, 0.5, 1.0, 1.0]);
    //        box_1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        box_1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 1.0, 1.0]);
    //        let box_1_id = self.pushrod.borrow_mut().add_widget_to_layout_manager(
    //            "Box1",
    //            Box::new(box_1),
    //            base_layout_id,
    //            make_origin_point(),
    //        );
    //
    //        let mut inner_box_1 = BoxWidget::new();
    //        inner_box_1.set_point(CONFIG_ORIGIN, 505, 105);
    //        inner_box_1.set_size(CONFIG_BODY_SIZE, 70, 60);
    //        inner_box_1.set_color(CONFIG_MAIN_COLOR, [0.75, 0.75, 1.0, 1.0]);
    //        inner_box_1.set_numeric(CONFIG_BORDER_WIDTH, 1);
    //        inner_box_1.set_color(CONFIG_BORDER_COLOR, [1.0, 0.0, 1.0, 1.0]);
    //        self.pushrod
    //            .borrow_mut()
    //            .add_widget_to_parent("Box2", Box::new(inner_box_1), box_1_id);
    //
    //        let mut inner_box_2 = BoxWidget::new();
    //        inner_box_2.set_point(CONFIG_ORIGIN, 585, 105);
    //        inner_box_2.set_size(CONFIG_BODY_SIZE, 70, 60);
    //        inner_box_2.set_color(CONFIG_MAIN_COLOR, [0.75, 0.25, 1.0, 1.0]);
    //        inner_box_2.set_numeric(CONFIG_BORDER_WIDTH, 1);
    //        inner_box_2.set_color(CONFIG_BORDER_COLOR, [1.0, 1.0, 0.0, 1.0]);
    //        self.pushrod
    //            .borrow_mut()
    //            .add_widget_to_parent("Box3", Box::new(inner_box_2), box_1_id);
    //
    //        let mut inner_box_3 = BoxWidget::new();
    //        inner_box_3.set_point(CONFIG_ORIGIN, 505, 190);
    //        inner_box_3.set_size(CONFIG_BODY_SIZE, 70, 60);
    //        inner_box_3.set_color(CONFIG_MAIN_COLOR, [0.25, 0.50, 0.75, 1.0]);
    //        inner_box_3.set_numeric(CONFIG_BORDER_WIDTH, 1);
    //        inner_box_3.set_color(CONFIG_BORDER_COLOR, [1.0, 0.50, 1.0, 1.0]);
    //        self.pushrod
    //            .borrow_mut()
    //            .add_widget_to_parent("Box4", Box::new(inner_box_3), box_1_id);
    //
    //        let mut inner_box_4 = BoxWidget::new();
    //        inner_box_4.set_point(CONFIG_ORIGIN, 585, 190);
    //        inner_box_4.set_size(CONFIG_BODY_SIZE, 70, 60);
    //        inner_box_4.set_color(CONFIG_MAIN_COLOR, [0.75, 0.50, 0.0, 1.0]);
    //        inner_box_4.set_numeric(CONFIG_BORDER_WIDTH, 1);
    //        inner_box_4.set_color(CONFIG_BORDER_COLOR, [0.50, 0.0, 0.25, 1.0]);
    //        self.pushrod
    //            .borrow_mut()
    //            .add_widget_to_parent("Box5", Box::new(inner_box_4), box_1_id);
    //
    //        let mut button = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Hide".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button.set_point(CONFIG_ORIGIN, 480, 290);
    //        button.set_size(CONFIG_BODY_SIZE, 200, 32);
    //        button.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_layout_manager(
    //            "HideButton3",
    //            Box::new(button),
    //            base_layout_id,
    //            make_origin_point(),
    //        );
    //    }
    //
    //    fn add_powered_by(&mut self) {
    //        let mut image_widget = ImageButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Powered By Rust!".to_string(),
    //            "rust-512x512.jpg".to_string(),
    //            18,
    //            TextJustify::Left,
    //        );
    //
    //        image_widget.set_point(CONFIG_ORIGIN, 570, 540);
    //        image_widget.set_size(CONFIG_BODY_SIZE, 220, 48);
    //        image_widget.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        image_widget.set_numeric(CONFIG_BORDER_WIDTH, 1);
    //        image_widget.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "RustImageButton",
    //            Box::new(image_widget),
    //        );
    //    }
    //
    //    fn add_progress(&mut self) {
    //        let mut progress_widget = ProgressWidget::new();
    //
    //        progress_widget.set_point(CONFIG_ORIGIN, 20, 360);
    //        progress_widget.set_size(CONFIG_BODY_SIZE, 230, 32);
    //        progress_widget.set_color(CONFIG_MAIN_COLOR, [1.0, 1.0, 1.0, 1.0]);
    //        progress_widget.set_color(CONFIG_SECONDARY_COLOR, [0.5, 0.5, 0.5, 1.0]);
    //        progress_widget.set_numeric(CONFIG_PROGRESS, 50);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "ProgressWidget",
    //            Box::new(progress_widget),
    //        );
    //
    //        let mut radio_1 = RadioButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "1".to_string(),
    //            20,
    //            TextJustify::Left,
    //            true,
    //        );
    //
    //        radio_1.set_point(CONFIG_ORIGIN, 20, 400);
    //        radio_1.set_size(CONFIG_BODY_SIZE, 75, 32);
    //        radio_1.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
    //        radio_1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        radio_1.set_numeric(CONFIG_WIDGET_GROUP_ID, 1);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "Radio1",
    //            Box::new(radio_1),
    //        );
    //
    //        let mut radio_2 = RadioButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "2".to_string(),
    //            20,
    //            TextJustify::Left,
    //            false,
    //        );
    //
    //        radio_2.set_point(CONFIG_ORIGIN, 100, 400);
    //        radio_2.set_size(CONFIG_BODY_SIZE, 75, 32);
    //        radio_2.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
    //        radio_2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        radio_2.set_numeric(CONFIG_WIDGET_GROUP_ID, 1);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "Radio2",
    //            Box::new(radio_2),
    //        );
    //
    //        let mut radio_3 = RadioButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "3".to_string(),
    //            20,
    //            TextJustify::Left,
    //            false,
    //        );
    //
    //        radio_3.set_point(CONFIG_ORIGIN, 180, 400);
    //        radio_3.set_size(CONFIG_BODY_SIZE, 75, 32);
    //        radio_3.set_color(CONFIG_MAIN_COLOR, [1.0; 4]);
    //        radio_3.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        radio_3.set_numeric(CONFIG_WIDGET_GROUP_ID, 1);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "Radio3",
    //            Box::new(radio_3),
    //        );
    //
    //        let mut progress_text = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "50%".to_string(),
    //            18,
    //            TextJustify::Left,
    //        );
    //
    //        progress_text.set_point(CONFIG_ORIGIN, 260, 360);
    //        progress_text.set_size(CONFIG_BODY_SIZE, 50, 32);
    //        progress_text.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "ProgressText1",
    //            Box::new(progress_text),
    //        );
    //
    //        let mut button1 = ToggleButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Animate".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button1.set_point(CONFIG_ORIGIN, 340, 360);
    //        button1.set_size(CONFIG_BODY_SIZE, 160, 32);
    //        button1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button1.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button1.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button1.set_toggle(CONFIG_SELECTED, true);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "AnimateButton1",
    //            Box::new(button1),
    //        );
    //
    //        let mut button2 = PushButtonWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Randomize".to_string(),
    //            18,
    //            TextJustify::Center,
    //        );
    //
    //        button2.set_point(CONFIG_ORIGIN, 520, 360);
    //        button2.set_size(CONFIG_BODY_SIZE, 160, 32);
    //        button2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        button2.set_numeric(CONFIG_BORDER_WIDTH, 2);
    //        button2.set_color(CONFIG_BORDER_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "RandomColorButton2",
    //            Box::new(button2),
    //        );
    //    }
    //
    //    fn add_timer(&mut self) {
    //        let mut timer = TimerWidget::new();
    //
    //        timer.set_numeric(CONFIG_TIMER_TIMEOUT, 100);
    //        timer.set_toggle(CONFIG_TIMER_ENABLED, true);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "TimerWidget1",
    //            Box::new(timer),
    //        );
    //    }
    //
    //    fn add_debugging(&mut self) {
    //        let mut check_widget = CheckboxWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Enable Debugging".to_string(),
    //            20,
    //            TextJustify::Left,
    //            true,
    //        );
    //        check_widget.set_point(CONFIG_ORIGIN, 20, 500);
    //        check_widget.set_size(CONFIG_BODY_SIZE, 400, 28);
    //        check_widget.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "DebugCheck1",
    //            Box::new(check_widget),
    //        );
    //
    //        let mut text_widget1 = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "Current Widget: 0".to_string(),
    //            20,
    //            TextJustify::Left,
    //        );
    //        text_widget1.set_point(CONFIG_ORIGIN, 20, 530);
    //        text_widget1.set_size(CONFIG_BODY_SIZE, 400, 28);
    //        text_widget1.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "DebugText1",
    //            Box::new(text_widget1),
    //        );
    //
    //        let mut text_widget2 = TextWidget::new(
    //            "assets/OpenSans-Regular.ttf".to_string(),
    //            "".to_string(),
    //            20,
    //            TextJustify::Left,
    //        );
    //        text_widget2.set_point(CONFIG_ORIGIN, 20, 560);
    //        text_widget2.set_size(CONFIG_BODY_SIZE, 400, 28);
    //        text_widget2.set_color(CONFIG_TEXT_COLOR, [0.0, 0.0, 0.0, 1.0]);
    //        self.pushrod.borrow_mut().add_widget_to_parent_by_name(
    //            "MainContainerWidget",
    //            "DebugText2",
    //            Box::new(text_widget2),
    //        );
    //    }

    fn build(&mut self) {
        self.add_hello_world();
        self.add_horizontal_layout();
        //        self.add_base_widget();
        //        self.add_box_widgets();
        //        self.add_powered_by();
        //        self.add_progress();
        //        self.add_timer();
        //        self.add_debugging();
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
    let window: GlfwWindow = WindowSettings::new("Pushrod Window", [800, 640])
        .opengl(OpenGL::V3_2)
        .resizable(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
    let mut app_window = SimpleWindow::new(Pushrod::new(window));

    app_window.run();
}
