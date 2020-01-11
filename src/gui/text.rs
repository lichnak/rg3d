use crate::core::{
    pool::Handle,
    math::{vec2::Vec2},
};
use std::{cell::RefCell, rc::Rc};
use crate::{
    gui::{
        VerticalAlignment,
        HorizontalAlignment,
        draw::DrawingContext,
        UserInterface,
        formatted_text::{FormattedText, FormattedTextBuilder},
        widget::{Widget, WidgetBuilder},
        UINode
    },
    resource::ttf::Font,
};
use crate::gui::Control;

pub struct Text {
    widget: Widget,
    need_update: bool,
    text: String,
    font: Rc<RefCell<Font>>,
    vertical_alignment: VerticalAlignment,
    horizontal_alignment: HorizontalAlignment,
    formatted_text: FormattedText,
}

impl Control for Text {
    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }

    fn draw(&mut self, drawing_context: &mut DrawingContext) {
        let bounds = self.widget.get_screen_bounds();
        if self.need_update {
            self.formatted_text.set_size(Vec2::new(bounds.w, bounds.h));
            self.formatted_text.set_text(self.text.as_str());
            self.formatted_text.set_color(self.widget.foreground());
            self.formatted_text.set_horizontal_alignment(self.horizontal_alignment);
            self.formatted_text.set_vertical_alignment(self.vertical_alignment);
            self.formatted_text.build();
            self.need_update = true; // TODO
        }
        drawing_context.draw_text(Vec2::new(bounds.x, bounds.y), &self.formatted_text);
    }
}

impl Text {
    pub fn set_text<P: AsRef<str>>(&mut self, text: P) -> &mut Self {
        self.text.clear();
        self.text += text.as_ref();
        self.need_update = true;
        self
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_font(&mut self, font: Rc<RefCell<Font>>) -> &mut Self {
        self.font = font;
        self.need_update = true;
        self
    }

    pub fn set_vertical_alignment(&mut self, valign: VerticalAlignment) -> &mut Self {
        self.vertical_alignment = valign;
        self
    }

    pub fn set_horizontal_alignment(&mut self, halign: HorizontalAlignment) -> &mut Self {
        self.horizontal_alignment = halign;
        self
    }
}

pub struct TextBuilder {
    widget_builder: WidgetBuilder,
    text: Option<String>,
    font: Option<Rc<RefCell<Font>>>,
    vertical_text_alignment: Option<VerticalAlignment>,
    horizontal_text_alignment: Option<HorizontalAlignment>,
}

impl TextBuilder {
    pub fn new(widget_builder: WidgetBuilder) -> Self {
        Self {
            widget_builder,
            text: None,
            font: None,
            vertical_text_alignment: None,
            horizontal_text_alignment: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        self
    }

    pub fn with_font(mut self, font: Rc<RefCell<Font>>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_opt_font(mut self, font: Option<Rc<RefCell<Font>>>) -> Self {
        self.font = font;
        self
    }

    pub fn with_vertical_text_alignment(mut self, valign: VerticalAlignment) -> Self {
        self.vertical_text_alignment = Some(valign);
        self
    }

    pub fn with_horizontal_text_alignment(mut self, halign: HorizontalAlignment) -> Self {
        self.horizontal_text_alignment = Some(halign);
        self
    }

    pub fn build(self, ui: &mut UserInterface) -> Handle<UINode> {
        let font =   if let Some(font) = self.font {
            font
        } else {
            ui.default_font.clone()
        };

        ui.add_node(Text {
            widget: self.widget_builder.build(),
            text: self.text.unwrap_or_default(),
            need_update: true,
            vertical_alignment: self.vertical_text_alignment.unwrap_or(VerticalAlignment::Top),
            horizontal_alignment: self.horizontal_text_alignment.unwrap_or(HorizontalAlignment::Left),
            formatted_text: FormattedTextBuilder::new().with_font(font.clone()).build(),
            font
        })
    }
}
