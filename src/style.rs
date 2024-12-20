use iced::Color;
use iced_widget::container;


/// A set of rules that dictate the styling of a [`Table`](crate::Table).
pub trait Catalog 
//where 
//iced_widget::container::Style: From<<iced::Theme as crate::style::Catalog>::Style>
{
    /// The supported style of the [`Catalog`].
    type Style: Default + Clone;

    // fn generate(style: &Self::Style, theme: &crate::Theme) -> iced_widget::container::Style;
    /// The header [`Style`](iced_widget::container::Style) of the [`Catalog`].
    fn body(&self, style: &Self::Style) -> container::Style;
    fn cell(&self, _row: usize, _col: usize) -> container::Style;
}

impl Catalog for iced_core::Theme {
    type Style = container::Style;

    fn body(&self, _style: &Self::Style) -> container::Style {
        container::Style {
            text_color: Some(self.extended_palette().background.strong.text),
            background: Some(self.extended_palette().background.strong.color.into()),
            ..Default::default()
        }
    }
    fn cell(&self, _row: usize, _col: usize) -> iced::widget::container::Style {
        container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.9))),
            ..Default::default()
        }
    }
    // fn generate(style: &Self::Style, _theme: &crate::Theme) -> iced_widget::container::Style {
    //     style.clone() // Clone the style directly
    // }
}


pub mod wrapper {
    use iced_core::{mouse::Cursor, Element, Length, Size, Vector, Widget};
    use iced_widget::{container, renderer::wgpu::{self, primitive::Renderer}};

    use crate::{Grid, GridMessage};

    pub fn style<'a, Message, Theme, Renderer>(
        content: &'a mut Grid<Message, Theme>,
        //impl Into<&'a mut Element<'a, Message, Theme, Renderer>>,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, Renderer>: From<Grid<Message, Theme>>
    {
        Element::new(
            Wrapper {
                content,
                target: Target::Style,
                style,
            }
        )
        //.into()
        
    }

    pub enum Target {
        Style,
        //Cell,
    }

    impl Target {
        fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            match self {
                Target::Style => theme.body(style),
            }
        }
    }
    
    impl<'a, Inner, Message, Theme, Renderer> From<Wrapper<'a, Inner, Theme>>
    for Element<'a, Message, Theme, Renderer>
        where
            Inner: Widget<Message, Theme, Renderer> + 'a,
            Renderer: iced_core::Renderer + 'a,
            Theme: super::Catalog + 'a,
            Message: 'a,
        {
            fn from(wrapper: Wrapper<'a, Inner, Theme>) -> Self {
                Element::new(wrapper)
            }
    }
    
    pub struct Wrapper<'a, Inner, Theme>
    where
        Inner: ?Sized,
        Theme: super::Catalog,
    {
        pub content: &'a Inner,
        pub target: Target,
        pub style: <Theme as super::Catalog>::Style,
    }
    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Grid<Message, Theme>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
        iced_core::Element<'a, Message, Theme, Renderer>: From<Grid<Message, Theme>>
        {
            fn size(&self) -> Size<Length> {
                // Convert to the correct type using `to_element()`
                let element = self.to_element();
                element.as_widget().size()
            }
            

            fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.layout(tree, renderer, limits)
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
            let appearance = Target::Style.appearance(theme, &self.style);
    
            renderer.fill_quad(
                iced_core::renderer::Quad {
                    bounds: layout.bounds(),
                    border: appearance.border,
                    shadow: Default::default(),
                },
                appearance
                    .background
                    .unwrap_or_else(|| iced_core::Color::TRANSPARENT.into()),
            );
        }
    }
    impl<'a, Inner, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Inner, Theme>
    where
        Inner: Widget<Message, Theme, Renderer> + ?Sized,
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
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
            // Custom rendering logic
    
            self.content
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    
        // Forward remaining methods to the wrapped widget
        fn tag(&self) -> iced_core::widget::tree::Tag {
            self.content.tag()
        }
    
        fn state(&self) -> iced_core::widget::tree::State {
            self.content.state()
        }
    
        fn children(&self) -> Vec<iced_core::widget::Tree> {
            self.content.children()
        }
    
        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            self.content.diff(tree)
        }
    
        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            self.content.operate(state, layout, renderer, operation);
        }
    
        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: iced_core::mouse::Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content.mouse_interaction(state, layout, cursor, viewport, renderer)
        }
    }
    

}

        // fn overlay<'b>(
        //     &'b mut self,
        //     state: &'b mut iced_core::widget::Tree,
        //     layout: iced_core::Layout<'_>,
        //     renderer: &Renderer,
        //     translation: Vector,
        // ) -> Option<iced_core::overlay::Element<'b, GridMessage, Theme, Renderer>> {
        //     self.content
        //         .as_widget_mut()
        //         .overlay(state, layout, renderer, translation)
        // }