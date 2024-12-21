//! Custom renderer trait with `fill_quad` and text rendering methods.
use iced::{
    Background, Border, Color, Font, Pixels, Point, Rectangle, Shadow, Size, Transformation, Vector
};
use iced_core::{layout, Layout};
use iced_widget::renderer::wgpu::{layer, Engine};

/// A polygon with four sides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    /// The bounds of the [`Quad`].
    pub bounds: Rectangle,

    /// The [`Border`] of the [`Quad`]. The border is drawn on the inside of the [`Quad`].
    pub border: Border,

    /// The [`Shadow`] of the [`Quad`].
    pub shadow: Shadow,
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            bounds: Rectangle::with_size(Size::ZERO),
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

/// The styling attributes of a [`Renderer`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The text color
    pub text_color: Color,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            text_color: Color::BLACK,
        }
    }
}

/// A renderer trait that provides methods to render graphical elements and text.
pub trait Renderer {

    /// Clears all recorded primitives in the [`Renderer`].
    fn clear(&mut self);

    /// Fills a [`Quad`] with the provided [`Background`].
    fn fill_quad(&mut self, quad: iced_core::renderer::Quad, background: Background);

    /// Draws text at the given position.
    fn fill_text(&mut self, text: iced_core::Text, position: Point, size: Pixels, color: Color, bounds: Rectangle);

}

/// A basic implementation of the `Renderer` trait.
pub struct BasicRenderer {
    default_font: Font,
    default_text_size: Pixels,
    layers: layer::Stack,
    engine: Engine,
}

impl Renderer for BasicRenderer {
    fn fill_quad(&mut self, quad: iced_core::renderer::Quad, background: Background) {
        //println!("Filling quad {:?} with background {:?}", quad, background.into());
        let (layer, transformation) = self.layers.current_mut();
        layer.draw_quad(quad, background, transformation);
    }

    fn clear(&mut self) {
        println!("Clearing renderer");
    }
    fn fill_text(
        &mut self,
        text: iced_core::Text,
        position: iced::Point, 
        pixels: iced::Pixels,
        color: iced::Color,
        bounds: Rectangle,
    ) {
        let (layer, transformation) = self.layers.current_mut();
        layer.draw_text(text, position, color, bounds, transformation);
    }
}

    // /// Measures the width of the given text.
    // fn measure_text(&self, text: &str, font: Font, size: Pixels) -> f32;

    // /// Returns the baseline offset of the given text.
    // fn text_baseline_offset(&self, font: Font, size: Pixels) -> f32;


    // fn start_layer(&mut self, bounds: Rectangle) {
    //     println!("Starting layer with bounds: {:?}", bounds);
    // }

    // fn end_layer(&mut self) {
    //     println!("Ending layer");
    // }

    // fn start_transformation(&mut self, transformation: Transformation) {
    //     println!("Starting transformation: {:?}", transformation);
    // }

    // fn end_transformation(&mut self) {
    //     println!("Ending transformation");
    // }
        // Starts recording a new layer.
    // fn start_layer(&mut self, bounds: Rectangle);

    // /// Ends recording a new layer.
    // fn end_layer(&mut self);

    // /// Draws the primitives recorded in the given closure in a new layer.
    // fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self)) {
    //     self.start_layer(bounds);
    //     f(self);
    //     self.end_layer();
    // }

    // /// Starts recording with a new [`Transformation`].
    // fn start_transformation(&mut self, transformation: Transformation);

    // /// Ends recording a new transformation.
    // fn end_transformation(&mut self);

    // /// Applies a [`Transformation`] to the primitives recorded in the given closure.
    // fn with_transformation(
    //     &mut self,
    //     transformation: Transformation,
    //     f: impl FnOnce(&mut Self),
    // ) {
    //     self.start_transformation(transformation);
    //     f(self);
    //     self.end_transformation();
    // }

    // /// Applies a translation to the primitives recorded in the given closure.
    // fn with_translation(
    //     &mut self,
    //     translation: Vector,
    //     f: impl FnOnce(&mut Self),
    // ) {
    //     self.with_transformation(
    //         Transformation::translate(translation.x, translation.y),
    //         f,
    //     );
    // }

// A headless renderer is a renderer that can render offscreen without
// a window or compositor.
// pub trait Headless {
//     /// Creates a new [`Headless`] renderer.
//     fn new(default_font: Font, default_text_size: Pixels) -> Self;

//     /// Draws offscreen into a screenshot, returning a collection of
//     /// bytes representing the rendered pixels in RGBA order.
//     fn screenshot(
//         &mut self,
//         size: Size<u32>,
//         scale_factor: f32,
//         background_color: Color,
//     ) -> Vec<u8>;
// }
