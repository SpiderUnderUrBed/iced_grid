pub use iced_core::renderer::Style as RendererStyle;
use iced_widget::{Theme};
use iced_core::{Color, Element, Length, Size, Widget, mouse::Cursor, Background};
use iced_core::layout::Limits;


#[derive(Default, Clone)]
pub trait Catalog {
    type Style: Default + Clone;

    // Optionally, you can add default values for style-related properties here if needed
    fn default_style() -> Self::Style;
}

// Implement Catalog fIsStyleor Theme with a Style containing background color
impl Catalog for Theme {
    type Style = Style; // Use our custom Style struct

    fn default_style() -> Self::Style {
        Style {
            text_color: Color::WHITE,
            background_color: Color::from_rgb(0.2, 0.8, 0.2), // Default background color
        }
    }
}

// Style struct containing text color and background color
#[derive(Default, Clone)]
pub struct Style {
    pub text_color: Color,        // Text color
    pub background_color: Color,  // Background color
}
trait IsStyle: Default + Clone {
    fn background_color() -> Color;
    fn text_color() -> Color;
}

pub(crate) mod wrapper {
    use super::*;
    use iced_core::{Element, Size, Widget, Renderer};
    use iced_widget::container;

    pub fn create_element<'a, Message, Theme, Renderer>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        style: <Theme as Catalog>::Style,
        target: Target,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: Catalog + 'a,
        Message: 'a,
    {
        Wrapper {
            content: content.into(),
            target,
            style,
        }
        .into()
    }

    
    enum Target {
        Row(usize), 
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as Catalog>::Style,
        ) -> Style
        where
            Theme: Catalog,
        {
            match self {
                Target::Row(_) => {
                    
                    Style {
                        text_color: Color::BLACK, 
                        background_color: Some(Color::from_rgb(0.1, 0.1, 0.1)), 
                    }
                }
            }
        }
    }

    
    pub struct Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: Catalog,
    {
        content: Element<'a, Message, Theme, Renderer>,
        target: Target,
        style: <Theme as Catalog>::Style,
    }

    
    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Wrapper<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer,
        Theme: Catalog,
    {
        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn layout(
            &self,
            state: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &Limits,
        ) -> iced_core::layout::Node {
            self.content.as_widget().layout(state, renderer, limits)
        }

        fn draw(
            &self,
            state: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
        ) {
            
            let background = Background::Color(self.style.background_color); 

            
            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: iced_core::Border {
                        width: 0.0, 
                        radius: 0.0.into(), 
                        color: Color::TRANSPARENT, 
                    },
                    shadow: Default::default(),
                },
                background,
            );

            
            self.content.as_widget().draw(state, renderer, theme, style, layout, cursor, viewport)
        }
    }

    
    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme, Renderer>> for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: Catalog + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme, Renderer>) -> Self {
            Element::new(wrapper)
        }
    }
}

