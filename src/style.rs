pub use iced_core::renderer::Style as RendererStyle;
use iced_widget::{Theme};
use iced_core::{Color, Element, Length, Size, Widget, mouse::Cursor, Background};
use iced_core::layout::Limits;


#[derive(Default, Clone)]
pub struct Style {
    pub text_color: Color,            
    pub background_color: Option<Color>,
}

impl Style {
    pub fn background_color_or_default(&self) -> Color {
        self.background_color
            .unwrap_or(Color::from_rgb(0.2, 0.8, 0.2))
    }
}
pub trait Catalog {
    type Style: Default + Clone;

    fn default_style() -> Self::Style;
}

impl Catalog for Theme {
    type Style = Style; 

    fn default_style() -> Self::Style {
        Style {
            text_color: Color::WHITE,
            background_color: None,
        }
    }
}


pub trait IsStyle: Default + Clone {
    fn background_color(&self) -> Color;
    fn text_color(&self) -> Color;
}

impl IsStyle for Style {
    fn background_color(&self) -> Color {
        self.background_color_or_default()
    }

    fn text_color(&self) -> Color {
        self.text_color
    }
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

