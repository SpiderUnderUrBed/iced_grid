use iced::Color;
use iced_widget::container;



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
    use iced_core::{layout::Node, mouse::Cursor, Element, Length, Size, Vector, Widget};
    use iced_widget::{container, renderer::wgpu::{self, primitive::Renderer}};

    use crate::{Cell, Grid, GridMessage};

    pub fn style<'a, Message, Theme, Renderer>(
        content: &'a mut Grid<Message, Theme>,
        
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
                
                let element = self.to_element();
                element.as_widget().size()
            }
            

            fn layout(
                &self,
                _tree: &mut iced_core::widget::Tree,
                _renderer: &Renderer,
                limits: &iced_core::layout::Limits,
            ) -> iced_core::layout::Node {
                
                let width = self.width;
                let height = self.height;
                let cell_width = width / self.rows[0].cells.len() as f32;
                let cell_height = height / self.rows.len() as f32;
            
                let mut children = Vec::new();
            
                
                for (row_index, row) in self.rows.iter().enumerate() {
                    for (cell_index, _cell) in row.cells.iter().enumerate() {
                        let size = iced_core::Size {
                            width: cell_width,
                            height: cell_height,
                        };
            
                        let position = iced_core::Point {
                            x: cell_index as f32 * cell_width,
                            y: row_index as f32 * cell_height,
                        };
                 
                        let mut child = iced_core::layout::Node::new(size).move_to(position);
            
                        children.push(child);
                    }
                }
                iced_core::layout::Node::with_children(iced_core::Size::new(width, height), children)
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
            
    
            self.content
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    
        
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

        
        
        
        
        
        
        
        
        
        
        