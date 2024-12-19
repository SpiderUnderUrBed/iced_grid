use iced::Color;
use iced_widget::container;
//use iced_core::Renderer;

pub trait Catalog 


{
    type Style: Default + Clone;
    
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
    {
        Wrapper {
            content,
            target: Target::Style,
            style,
        }
        .into()
    }

    pub enum Target {
        Style,
        
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


    pub struct Wrapper<'a, Message, Theme>
    where
        Theme: super::Catalog,
    {
        pub content: &'a Grid<Message, Theme>,
        pub target: Target,
        pub style: <Theme as super::Catalog>::Style,
    }
    impl<'a, Message, Theme, Renderer> From<Wrapper<'a, Message, Theme>>
        for Element<'a, Message, Theme, Renderer>
    where
        Renderer: iced_core::Renderer + 'a,
        Theme: super::Catalog + 'a,
        Message: 'a,
    {
        fn from(wrapper: Wrapper<'a, Message, Theme>) -> Self {
            Element::new(wrapper)
        }
    }
//&mut Grid<Message, MyTheme>
    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Grid<Message, Theme>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        fn size(&self) -> Size<Length> {
        todo!()
    }
    
        fn layout(
        &self,
        tree: &mut iced_core::widget::Tree,
        renderer: &Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        todo!()
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
        todo!()
    }
    }

    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
        for Wrapper<'a, Message, Theme>
    where
        Renderer: iced_core::Renderer,
        Theme: super::Catalog,
    {
        fn size(&self) -> Size<Length> {
            <Grid<Message, Theme> as iced_core::Widget<Message, Theme, Renderer>>::size(self.content)
        }

        fn layout(
            &self,
            state: &mut iced_core::widget::Tree,
            renderer: &Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.layout(state, renderer, limits)
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
            let appearance = self.target.appearance(theme, &self.style);

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

            let text_style = appearance
                .text_color
                .map(|color| iced_core::renderer::Style { text_color: color })
                .unwrap_or(*style);

            self.content
                
                .draw(state, renderer, theme, &text_style, layout, cursor, viewport);
        }

        fn tag(&self) -> iced_core::widget::tree::Tag {
            <Grid<Message, Theme> as iced_core::Widget<Message, Theme, Renderer>>::tag(self.content)
        }

        fn state(&self) -> iced_core::widget::tree::State {
            <Grid<Message, Theme> as iced_core::Widget<Message, Theme, Renderer>>::state(self.content)
        }

        fn children(&self) -> Vec<iced_core::widget::Tree> {
            <Grid<Message, Theme> as iced_core::Widget<Message, Theme, Renderer>>::children(self.content)
        }

        fn diff(&self, tree: &mut iced_core::widget::Tree) {
            <Grid<Message, Theme> as iced_core::Widget<Message, Theme, Renderer>>::diff(self.content, tree)
        }

        fn operate(
            &self,
            state: &mut iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced_core::widget::Operation,
        ) {
            self.content
                
                .operate(state, layout, renderer, operation);
        }

        // fn on_event(
        //     &mut self,
        //     state: &mut iced_core::widget::Tree,
        //     event: iced_core::Event,
        //     layout: iced_core::Layout<'_>,
        //     cursor: Cursor,
        //     renderer: &Renderer,
        //     clipboard: &mut dyn iced_core::Clipboard,
        //     shell: &mut iced_core::Shell<'_, Message>,
        //     viewport: &iced_core::Rectangle,
        // ) -> iced_core::event::Status {
        //     self.content.on_event(
        //         state, event, layout, cursor, renderer, clipboard, shell, viewport,
        //     )
        // }

        fn mouse_interaction(
            &self,
            state: &iced_core::widget::Tree,
            layout: iced_core::Layout<'_>,
            cursor: Cursor,
            viewport: &iced_core::Rectangle,
            renderer: &Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content
                
                .mouse_interaction(state, layout, cursor, viewport, renderer)
        }

        // fn overlay<'b>(
        //     &'b mut self,
        //     state: &'b mut iced_core::widget::Tree,
        //     layout: iced_core::Layout<'_>,
        //     renderer: &Renderer,
        //     translation: Vector,
        // ) -> Option<iced_core::overlay::Element<'b, Message, Theme, Renderer>> {
        //     self.content
        //         .overlay(state, layout, renderer, translation)
        // }
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