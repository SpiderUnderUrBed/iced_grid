use iced::Color;
use iced_core::Widget;
use iced_widget::container;
use wrapper::Wrapper;
 
pub trait Catalog 
 
 
{
    // fn from_style(style: Self::Style) -> Self;
    type Style: Default + Clone;
    type Themes;
    
    fn body(&self, style: &Self::Style) -> Self::Style;
    fn cell(&self, _row: usize, _col: usize) -> Self::Style;
    fn resolve_theme(&self) -> Self::Themes;
}
 
impl Catalog for iced_core::Theme {
    type Style = container::Style;
    type Themes = iced_core::Theme;

    fn body(&self, _style: &Self::Style) -> Self::Style {
        Self::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }
    fn resolve_theme(&self) -> Self::Themes {
        iced_core::Theme::Dark
    }
    fn cell(&self, _row: usize, _col: usize) -> Self::Style {
        Self::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.9))),
            ..Default::default()
        }
    }
}

 
pub mod wrapper {
    use iced::{Font, Pixels, Theme};
    use iced_core::{layout::Node, mouse::Cursor, text::Renderer, Element, Length, Size, Vector, Widget};
    use iced_widget::{button, canvas::path::lyon_path::geom::euclid::default, container, renderer::wgpu::{self}};
 
    use crate::{Cell, Grid, GridMessage};

    use super::renderer;
 
    pub fn style<'a, Message, Theme, Renderer>
    (
        content: &'a mut Grid<Message, Theme>,
        theme: Theme,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, crate::Renderer>
    where
        //Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog<Themes = iced_core::Theme, Style = iced_widget::container::Style> + Clone + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, crate::Renderer>: From<Grid<Message, Theme>>
    {
        Element::new(
            Wrapper {
                content,
                target: Style,
                theme,
                style: content.style.clone()
            }
        )
 
 
    }
    pub struct Style;

    impl Style {
        // Implement methods for Style, similar to what was done in Target::Style
        pub fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
           // Theme: super::Catalog,
            Theme: super::Catalog<Style = iced_widget::container::Style>,
        {
            theme.body(style) // This can stay the same if it applies to the new Style struct
        }
    }

 
    impl<'a, Inner, Message, Theme, Renderer> From<Wrapper<'a, Inner, Theme>>
    for Element<'a, Message, Theme, Renderer>
        where
            Inner: Widget<Message, Theme, Renderer> + 'a,
            Renderer: iced_core::Renderer + 'a,
            Theme: super::Catalog<Themes = iced_core::Theme> + 'a,
            Message: 'a,
        {
            fn from(wrapper: Wrapper<'a, Inner, Theme>) -> Self {
                Element::new(wrapper)
            }
    }
    pub struct Wrapper<'a, Inner, Theme>
    where
        Inner: ?Sized,
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        pub content: &'a Inner,
       // pub target: Target,
        pub theme: Theme,
        pub style: <Theme as super::Catalog>::Style,
        pub target: Style,
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Grid<Message, Theme>
    where
        Renderer: iced_core::Renderer,
        //Renderer: iced::advanced::Renderer + iced_core::text::Renderer<Font = iced_core::Font>,
        Theme: super::Catalog<Style = iced_widget::container::Style, Themes = iced::Theme> + Clone,
    {
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.to_element().as_widget().layout(tree, renderer, limits)
        }
            fn size(&self) -> Size<Length> {
                //let element = self.to_element();
                self.to_element().as_widget().size()
            }
            
            
 
            fn draw(
                &self,
                tree: &iced_core::widget::Tree,
                renderer: &mut Renderer,
                theme: &Theme,
                style: &iced_core::renderer::Style,
                layout: iced_core::Layout<'_>,
                cursor: iced_core::mouse::Cursor,
                viewport: &iced::Rectangle,
            ) {
                self.to_element().as_widget().draw(tree, renderer, theme, style, layout, cursor, viewport);
            }
            
            
                // Optionally draw additional content by delegating
                // self.to_element().as_widget().draw(tree, renderer, theme, style, layout, cursor, viewport);
            }
                
    }

    impl<'a, Inner, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Inner, Theme>
    where
        Inner: Widget<Message, Theme, Renderer> + ?Sized,
        Renderer: iced_core::Renderer,
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        fn size(&self) -> iced_core::Size<iced_core::Length> {
            self.content.size()
        }
 
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.layout(tree, renderer, limits)
        }
 
        fn draw(
            &self,
            tree: &iced_core::widget::Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &iced_core::renderer::Style,
            layout: iced_core::Layout<'_>,
            cursor: iced_core::mouse::Cursor,
            viewport: &iced_core::Rectangle,
        ) {
 
 
            self.content
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
 

    }
