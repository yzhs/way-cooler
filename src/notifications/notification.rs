use std::iter;
use std::fmt::{self, Debug};
use std::cmp::{Eq, PartialEq};
use rustwlc::{Geometry, Size, WlcOutput};
use rustwlc::render::{calculate_stride};
use cairo::{ImageSurface, Format};

use uuid::Uuid;
use ::registry;
use ::render::{Color, Renderable};


/// A notification to the user that an event has happened.
pub struct Notification {
    /// The surface that contains the bytes we give wlc to draw.
    surface: ImageSurface,
    /// The output that the notification is displayed on
    output: WlcOutput,
    /// The string that's displayed as the "title"
    /// or "header" of the notificaiton.
    title: String,
    /// The string that's displayed as the body text of the notification.
    text: String,
    /// The geometry of where the buffer is drawn.
    ///
    /// The width and height should be the same size as the surface's
    /// width * height * 4. If it's not, then the buffer will fail to draw.
    geometry: Geometry,
    /// The background color that the text is overlayed on to.
    background_color: Color,
    /// The color that the header/title text is displayed in.
    title_color: Color,
    /// The color that the body text is displayed in.
    text_color: Color
}


impl Renderable for Notification {
    fn new(mut geometry: Geometry, output: WlcOutput) -> Option<Self> {
        None
    }

    fn get_surface(&mut self) -> &mut ImageSurface {
        &mut self.surface
    }

    fn get_geometry(&self) -> Geometry {
        self.geometry
    }

    fn set_geometry(&mut self, geometry: Geometry) {
        self.geometry = geometry;
    }

    fn get_output(&self) -> WlcOutput {
        self.output
    }

    /// Updates/Creates the underlying geometry for the surface/buffer.
    ///
    /// This causes a reallocation of the buffer, do not call this
    /// in a tight loop unless you want memory fragmentation and
    /// bad performance.
    fn reallocate_buffer(mut self, mut geometry: Geometry) -> Option<Self>{
        None
    }
}
