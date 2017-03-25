use std::ops::{Deref, DerefMut};
use rustwlc::{Geometry, Size, Point};
use super::Notification;
use ::render::{BaseDraw, Drawable, DrawErr, Renderable};

// Context, set some bits -> Layout, set some actual interesting bits,
// and then call Layout::get_text(layout);

pub struct NotificationDraw {
    base: BaseDraw<Notification>
}


impl NotificationDraw {
    pub fn new(base: BaseDraw<Notification>) -> Self {
        NotificationDraw {
            base: base
        }
    }

    fn draw_background(mut self, geo: Geometry) -> Result<Self, DrawErr<Notification>> {
        let background_color = self.base.inner().background_color();
        self.base.set_color_source(background_color);
        self.base = try!(self.base.check_cairo());
        self.base.rectangle(0f64, 0f64, geo.size.w as f64, geo.size.h as f64);
        self.base = try!(self.base.check_cairo());
        self.base.fill();
        self.base = try!(self.base.check_cairo());
        Ok(self)
    }

    fn draw_title_text(mut self, geo: Geometry) -> Result<Self, DrawErr<Notification>> {
        // TODO Get the size of the extent of the string w/ self.base.text_extents(str)
        // Then, with that calculated value set the size of the buffer (this should
        // all be done in Notification::allocate)
        // For strings that make a size bigger than 1/4 of the screen,
        // truncate them with "..." at the end.
        // with that size, draw the string.

        // For the body of the text, do the same thing but instead of truncating
        // split it up into a Vec<String>, which is limited in the y value  by
        // 1/4 the screen height. E.g after that add ...
        // The line size can be calculated by summing the y values of the extents
        // of all the lines.
        let title: String =
            "Test title a really really \n really really really long title".into();
        use cairo::{Glyph, TextCluster};
        use cairo::enums::TextClusterFlags;

        let title_color = self.base.inner().title_color();
        self.base.set_color_source(title_color);
        self.base.move_to(10.0, 10.0);
        self.base = try!(self.base.check_cairo());
        let bytes = title.bytes().len() as i32;
        let glyphs = title.chars().count() as i32;
        self.base.show_text_glyphs("a",
                                   &[Glyph { index: 97, x: 20.0, y: 20.0}],
                                   &[TextCluster {
                                       num_bytes: 1,
                                       num_glyphs: 1
                                   }],
                                   TextClusterFlags::None);
        self.base = try!(self.base.check_cairo());
        Ok(self)
    }

    fn draw_body_text(mut self, geo: Geometry) -> Result<Self, DrawErr<Notification>> {
        Ok(self)
    }
}


impl Drawable<Notification> for NotificationDraw {
    fn draw(mut self, view_g: Geometry) -> Result<Notification, DrawErr<Notification>> {
        self.base.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        self.base.paint();
        let notification_geo = self.base.inner().get_geometry();
        self = self.draw_background(notification_geo)?
                   .draw_title_text(notification_geo)?
                   .draw_body_text(notification_geo)?;
        Ok(self.base.finish(notification_geo))
    }
}
