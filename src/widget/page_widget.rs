// Page Widget
// Extensible Widget that stores pages of content, hiding and showing them on demand
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

use crate::widget::config::*;
use crate::widget::widget::*;

/// Contains a series of `CanvasWidget` (top level) widgets that can be selected for display at any
/// given time, but only one can be swapped at a time.
pub struct PageWidget {
    config: Configurable,
    containers: Vec<CanvasWidget>,
    widget_id: i32,
    callbacks: DefaultWidgetCallbacks,
    current_page: i32,
}

impl PageWidget {
    /// Constructor.  Requires the name of the font, the text to display, the image name to display, the size of the font,
    /// and the font justification when rendered.  Images are loaded from the `assets/`
    /// directory.
    pub fn new(num_pages: u8) -> Self {
        let mut container_vec = Vec::new();

        for _i in 0..num_pages {
            container_vec.push(CanvasWidget::new());
        }

        Self {
            config: Configurable::new(),
            containers: Vec::new(),
            widget_id: 0,
            callbacks: DefaultWidgetCallbacks::new(),
            current_page: 0,
        }
    }

    pub fn get_current_page(&self) -> i32 {
        self.current_page
    }

    pub fn set_current_page(&mut self, page: i32) {
        self.current_page = page;

        // Hide all pages
        self.containers
            .iter_mut()
            .for_each(|c| c.set_toggle(CONFIG_WIDGET_HIDDEN, true));

        // Show specified page.
        self.containers[page as usize].set_toggle(CONFIG_WIDGET_HIDDEN, false);

        self.invalidate();
    }

    pub fn get_canvas_for_page(&mut self, page: i32) -> &mut CanvasWidget {
        &mut self.containers[page as usize]
    }
}

impl Drawable for PageWidget {}

impl InjectableSystemEvents for PageWidget {}

impl InjectableCustomEvents for PageWidget {}

impl Widget for PageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn set_config(&mut self, config: u8, config_value: Config) {
        self.config().set(config, config_value.clone());

        self.containers
            .iter_mut()
            .for_each(|c| c.set_config(config, config_value.clone()));
    }

    fn set_widget_id(&mut self, widget_id: i32) {
        self.widget_id = widget_id;
    }

    fn get_widget_id(&mut self) -> i32 {
        self.widget_id
    }

    fn get_injectable_custom_events(&mut self) -> &mut dyn InjectableCustomEvents {
        self
    }

    fn get_injectable_system_events(&mut self) -> &mut dyn InjectableSystemEvents {
        self
    }

    fn get_drawable(&mut self) -> &mut dyn Drawable {
        self
    }

    fn get_callbacks(&mut self) -> &mut DefaultWidgetCallbacks {
        &mut self.callbacks
    }
}
