use crate::{Rltk, Rect, RGB, to_cp437, Console};
mod theme;
pub use theme::*;
mod widgets;
pub use widgets::{SolidBackground, StatusBar, StatusBarText, Window};
mod element_store;
use element_store::*;

pub struct TextUI {
    element_store : ElementStore,
    theme : Theme,
    base_element : usize
}

impl TextUI {
    pub fn new(theme : Theme) -> TextUI {
        TextUI{
            element_store : ElementStore::new(),
            theme,
            base_element : 0
        }
    }

    pub fn get_id<S : ToString>(&self, id : S) -> Option<&usize> {
        self.element_store.get_id(id)
    }

    pub fn add_explicit<S : ToString>(&mut self, key: S, element : Box<dyn Element>, physical_bounds : Rect, placement : Placement, parent_id_option : Option<usize>) -> usize {
        self.element_store.add_element(key.to_string(), element, parent_id_option, physical_bounds, placement)
    }

    pub fn add<S : ToString>(&mut self, ctx : &mut Rltk, widget : WidgetType, key : S, parent : S) -> &mut Self {
        let parent_id = self.element_store.get_id(parent.to_string());
        let parent_v = match parent_id {
            None => None,
            Some(pid) => Some(*pid)
        };
        self.add_return_id(ctx, widget, key, parent_v);
        self
    }

    pub fn add_return_id<S : ToString>(&mut self, ctx : &mut Rltk, widget : WidgetType, key : S, parent : Option<usize>) -> usize {
        match widget {
            WidgetType::ScreenBackground => {
                let screen_size = ctx.get_char_size();
                let bounds = Rect::new(0, 0, screen_size.0 as i32, screen_size.1 as i32);
                self.add_explicit(
                    key, 
                    SolidBackground::screen_background(&self.theme), 
                    bounds, 
                    Placement::Absolute,
                    parent
                )
            }
            WidgetType::StatusBar => {
                let parent_bounds = self.element_store.get_physical_bounds(parent.unwrap());
                self.add_explicit(
                    key, 
                    StatusBar::new(&self.theme),
                    Rect::new(parent_bounds.x1, parent_bounds.y2 - 1, parent_bounds.width(), 1),
                    Placement::Relative,
                    parent
                )
            }
            WidgetType::StatusText{text} => {
                let parent_id = parent.unwrap();
                let parent_bounds = self.element_store.get_physical_bounds(parent_id);
                let x = 1 + parent_bounds.x1 + self.element_store.calc_child_width(parent_id);
                let width = text.len() as i32 + 1;
                self.add_explicit(
                    key, 
                    StatusBarText::new(&self.theme, text),
                    Rect::new(x, 0, width, 1),
                    Placement::Relative,
                    parent
                )
            }
            WidgetType::Window{pos, title} => {
                self.add_explicit(
                    key, 
                    Window::new(&self.theme, title),
                    Rect::new(pos.x1,pos.y1,pos.width(),pos.height()),
                    Placement::Relative,
                    parent
                )
            }
        }
    }

    pub fn set_base<S : ToString>(&mut self, id : S) -> &mut Self {
        let key = self.element_store.get_id(id).unwrap();
        self.base_element = *key;
        self
    }

    pub fn render(&self, ctx : &mut Rltk) {
        self.element_store.render(ctx, self.base_element);
    }    

    #[allow(clippy::borrowed_box)]
    pub fn element_by_id(&mut self, id : usize) -> &mut Box<dyn Element> {
        self.element_store.element_by_id(id)
    }
}

pub enum WidgetType {
    ScreenBackground,
    StatusBar,
    StatusText{ text : String },
    Window { pos : Rect, title : String }
}