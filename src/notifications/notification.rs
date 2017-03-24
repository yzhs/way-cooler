use std::iter;
use std::fmt::{self, Debug};
use std::cmp::{Eq, PartialEq};
use rustwlc::{Geometry, Size, WlcOutput};
use rustwlc::render::{calculate_stride};
use cairo::{ImageSurface, Format};

use uuid::Uuid;
use ::registry;
use ::render::{Color, Renderable, drop_data};


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
    ///
    /// If unspecified, the default from the registry is used.
    background_color: Option<Color>,
    /// The color that the header/title text is displayed in.
    ///
    /// If unspecified, the default from the registry is used.
    title_color: Option<Color>,
    /// The color that the body text is displayed in.
    ///
    /// If unspecified, the default from the registry is used.
    text_color: Option<Color>
}


impl Renderable for Notification {
    fn new(mut geometry: Geometry, output: WlcOutput) -> Option<Self> {
        Notification::allocate_buffer(geometry, drop_data)
            .and_then(|surface| {
                Some(Notification {
                    surface: surface,
                    output: output,
                    title: "".into(),
                    text: "".into(),
                    geometry: geometry,
                    background_color: None,
                    title_color: None,
                    text_color: None
                })})
    }

    fn allocate_buffer<F>(geometry: Geometry, drop_f: F) -> Option<ImageSurface>
        where F: FnOnce(Box<[u8]>) + 'static {
        panic!()
    }

    fn set_surface(&mut self, surface: ImageSurface) {
        self.surface = surface;
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
}
