use iced::Color;
use iced_widget::container;
 
 
 
pub trait Catalog 
 
 
{
    
    type Style: Default + Clone;
    type Themes;
    
    fn body(&self, style: &Self::Style) -> container::Style;
    fn cell(&self, _row: usize, _col: usize) -> container::Style;
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
        iced_core::Theme::CatppuccinLatte
    }
    fn cell(&self, _row: usize, _col: usize) -> Self::Style {
        Self::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.6, 0.6, 0.9))),
            ..Default::default()
        }
    }
}

 
pub mod wrapper {
    use std::borrow::{Borrow, BorrowMut};

    use iced::{border::Radius, mouse, theme::palette::Background, Rectangle, Theme};
    use iced_core::{layout::Node, mouse::Cursor, widget::Tree, Element, Layout, Length, Size, Vector, Widget};
    use iced_widget::{button, container, renderer::wgpu::{self, primitive::Renderer}, Button, Container};
 
    use crate::{Cell, CellMessage, Grid, GridMessage};
 
    fn traverse_tree(tree: &iced_core::widget::Tree) {
        
        
        println!("start");
        println!("{:?}", tree.state);
        println!("{:?}", tree.tag);
        println!("end");
        
        for child in &tree.children {
            traverse_tree(child);
        }
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn construct_new_tree(tree: &iced_core::widget::Tree) -> iced_core::widget::Tree {
        let mut new_tree = iced_core::widget::Tree::empty(); 
    
        
        for (index, child) in tree.children.iter().enumerate() {
            if index > 0 {
                
                let new_child = construct_new_tree(child);
                new_tree.children.push(new_child);
            }
        }
    
        new_tree
    }
    

//     pub fn style<'a, Message, Theme, Renderer>
//     (
//         content: &'a mut Grid<Message, Theme>,
//         theme: Theme,
//         style: <Theme as super::Catalog>::Style,
//     ) -> Element<'a, Message, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>>
//     where
//   //  Renderer: iced_core::Renderer + iced_core::text::Renderer,
        
//         Theme: super::Catalog<Themes = iced_core::Theme> + iced_widget::text::Catalog  + iced_widget::container::Catalog + Clone + 'a,
//         Message: 'a,
//         iced_core::Element<'a, Message, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>>: From<Grid<Message, Theme>>
//     {
//         let return_element: Element<'_, _, Theme, iced_widget::renderer::fallback::Renderer<_, _>> = Element::new(
//             Wrapper {
//                 content: Box::new(content),
//                 target: Style,
//                 theme,
//                 style: content.style.clone()
//             }
//         );
//         return_element
 
 
//     }
    pub struct Style;

    impl Style {
        
        pub fn appearance<Theme>(
            &self,
            theme: &Theme,
            style: &<Theme as super::Catalog>::Style,
        ) -> container::Style
        where
            Theme: super::Catalog,
        {
            theme.body(style) 
        }
    }
    
    impl<'a, Inner, Message, Theme> From<Wrapper<'a, Inner, Theme>>
    for Element<'a, Message, Theme, iced_widget::Renderer>
        where
            Inner: Widget<Message, Theme, iced_widget::Renderer> + 'a,
            
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
        pub content: Box<&'a Inner>,
       
        pub theme: Theme,
        pub style: <Theme as super::Catalog>::Style,
        pub target: Style,
    }
    impl<'a, Message, Theme, Renderer> Widget<Message, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>> for Grid<Message, Theme>
    where
        Theme: super::Catalog<Themes = iced_core::Theme> + iced_widget::container::Catalog
        + Clone + iced_widget::text::Catalog,
        iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>,
       Renderer: iced_core::Renderer + iced_core::text::Renderer,
        
    {
            fn size(&self) -> Size<Length> {
               
               
               Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
            }
            
        

            fn layout(
                &self,
                tree: &mut Tree,
                renderer: &iced_widget::renderer::fallback::Renderer<Renderer,Renderer>,
                limits: &iced_core::layout::Limits,
            ) -> iced_core::layout::Node {
                let max_size = limits.max();
                let width = self.width.min(max_size.width);
                let height = self.height.min(max_size.height);
                        
                let rows = self.rows.len();
                let cols = self.rows.get(0).map_or(0, |row| row.cells.len());
        
                if rows == 0 || cols == 0 {
                    println!("No rows or columns, returning ZERO node.");
                    return iced_core::layout::Node::new(iced_core::Size::ZERO);
                }
        
                let cell_width: f32 = 20.0;
                let cell_height: f32 = 20.0;
                let column_distance: f32 = 20.0;
                let row_distance: f32 = 20.0;                

                self.rows.iter().for_each(|row| {
                    if !row.cells.is_empty() {
                        row.cells.iter().for_each(|cell: &Cell<'_>| {
                           
                            match cell {
                                Cell::Text(text) => {
                                    let widget: &dyn Widget<Message, _, _> = text;
                                    tree.diff(widget);
                                }
                                Cell::Button(button) => {
                                    let widget: &dyn Widget<_, _, _> = button;
                                    tree.diff(widget);
                                }
                                Cell::Container(container) => {
                                    let widget: &dyn Widget<_, _, _> = container;
                                    tree.diff(widget);
                                }
                            }
                        });
                    }
                });
        
                let mut children = Vec::new();
                for (row_index, row) in self.rows.iter().enumerate() {
                    if row.cells.is_empty() {
                       
                        continue;
                    }
        
                    for (col_index, cell) in row.cells.iter().enumerate() {
                        let index = row_index * cols + col_index;
        
                        
        
                        let position = iced_core::Point {
                            x: col_index as f32 * (cell_width + column_distance),
                            y: row_index as f32 * (cell_height + row_distance),
                        };
        
                        let size = iced_core::Size::new(cell_width, cell_height);
        
                        match cell {
                            
                            Cell::Container(container) => {
                                
                                println!(
                                    "Creating layout node for container at position={:?}, size={:?}",
                                    position, size
                                );

                                let mut node_children = Vec::new(); 
                                
                                node_children.extend((0..3).map(|i| {
                                   
                                    let child_size = Size::new(50.0, 50.0);
                                    let mut child_node = Node::new(child_size);
                                
                                    
                                    let x_position = (i as f32 * 60.0) % size.width;
                                    let y_position = (i as f32 * 60.0) % size.height;
                                
                                    child_node.move_to(iced::Point::new(x_position, y_position))
                                }));
                                
                                let container_node = Node::with_children(size, node_children);
                              
                                children.push(container_node.move_to(position));
                                
                            }
                            _ => {
                                
                                println!(
                                    "Creating layout node for non-container cell at position={:?}, size={:?}",
                                    position, size
                                );
                                let dummy_node = iced_core::layout::Node::new(size);
                                children.push(dummy_node.move_to(position));
                            }
                        }
                    }
                }
                
                
                let nodes = iced_core::layout::Node::with_children(iced_core::Size::new(width, height), children);
                println!("{:#?}", nodes);
                nodes
            }
            
            fn draw(
                &self,
                state: &Tree,
                renderer: &mut iced_widget::renderer::fallback::Renderer<Renderer,Renderer>,
                theme: &Theme,
                style: &iced_core::renderer::Style,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                viewport: &Rectangle,
            ) {
                //let mut elements: Vec<Element<CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>>> = Vec::new();
                let mut elements: Vec<Element<CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>>> = Vec::new();

                for row in &self.rows {
                    for cell in &row.cells {
                        let element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_, _>> = match cell {
                            Cell::Container(container2) => {
                             //   let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = Element::new(*container2);
                             let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>> = Element::<CellMessage, Theme, iced_widget::renderer::fallback::Renderer<iced::Renderer, iced_tiny_skia::Renderer>
                             >::new(container("Es"));
                                return_element
                            },
                            _ => {
                                let newer_container: Container<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_, _>> = container("E");
                               // let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = Element::new(newer_container);
                               let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>> = Element::new(newer_container);
                                return_element
                            },
                        };
                        elements.push(element);
                    }
                }
                
            
                let new_container: Container<'_, Message, iced::Theme, iced_widget::renderer::fallback::Renderer<iced::Renderer, iced_tiny_skia::Renderer>> = container("Test");
            
                
                let new_states: Vec<_> = self
                    .rows
                    .iter()
                    .flat_map(|row| {
                        row.cells.iter().map(|cell: &Cell<'_>| match cell {
                            Cell::Container(container) => {
                                //container.state()
                                iced_core::widget::Tree { 
                                    tag: container.tag(), 
                                    state: container.state(), 
                                    children: container.children() 
                                }
                            //    new_container.tag()
                             },
                            _ => {
                                // new_container.state()
                                iced_core::widget::Tree { 
                                    tag: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::tag(&new_container), 
                                    state: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::state(&new_container), 
                                    children: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::children(&new_container) 
                                }
                            }
                        })
                    })
                    .collect();
            
                
                // let new_root_state = Tree {
                //     tag: new_container.tag(), 
                //     state: new_container.state(), 
                //     children: new_states, 
                // };
            
                
                for ((element, state), l) in elements
                    .iter()
                    .zip(new_states.iter())  
                    .zip(layout.children())
                {

                    if l.children().next().is_none() {                        // println!("Skipping draw for layout: {:?}", l);
                        continue;
                    }
                    println!("a");
                    // println!("{:#?}", element.as_widget().layout(state, renderer, limits));
                    println!("{:#?}", element.as_widget().state());
                   // let new_renderer: &mut iced_widget::renderer::fallback::Renderer<iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>, iced_tiny_skia::Renderer> = renderer;
                //    let new_element: &Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = element;

                //    new_element
                //         .as_widget()
                //         .draw(state, renderer, &iced::Theme::Dark, style, l, cursor, viewport);
                    let test_element: Element<'_, _, Theme, iced_widget::renderer::fallback::Renderer<Renderer, Renderer>> = Element::<Message, Theme, iced_widget::renderer::fallback::Renderer<Renderer, Renderer>>::new(iced::widget::Text::new("E"));
                    test_element.as_widget().draw(state, renderer, theme, style, layout, cursor, viewport);
                }
            }
            
    }

    impl<'a, Inner, Theme, Renderer> Borrow<dyn iced_core::Widget<CellMessage, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>> + 'a>
    for Wrapper<'a, Inner, Theme>
        where
            Inner: crate::style::Catalog<Themes = iced::Theme>,
            Wrapper<'a, Inner, Theme>: Widget<CellMessage, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>>, 
            Renderer: iced_core::Renderer + 'a,
            Theme: crate::style::Catalog<Themes = iced_core::Theme> + 'a,
        {
            fn borrow(&self) -> &(dyn iced_core::Widget<CellMessage, Theme, iced_widget::renderer::fallback::Renderer<Renderer,Renderer>> + 'a) {
                self
            }
        }
    
    
    
    
    
    
    
    
    
    
    
    
                
    impl<'a, Inner, Message, Theme> Widget<Message, Theme, iced_widget::Renderer>
        for Wrapper<'_, Inner, Theme>
    where
        Inner: Widget<Message, Theme, iced_widget::Renderer> + ?Sized,
        
        Theme: super::Catalog<Themes = iced_core::Theme>,
    {
        fn size(&self) -> iced_core::Size<iced_core::Length> {
            self.content.size()
        }
 
        fn layout(
            &self,
            tree: &mut iced_core::widget::Tree,
            renderer: &iced_widget::Renderer,
            limits: &iced_core::layout::Limits,
        ) -> iced_core::layout::Node {
            self.content.layout(tree, renderer, limits)
        }
 
        fn draw(
            &self,
            tree: &iced_core::widget::Tree,
            renderer: &mut iced_widget::Renderer,
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
            renderer: &iced_widget::Renderer,
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
            renderer: &iced_widget::Renderer,
        ) -> iced_core::mouse::Interaction {
            self.content.mouse_interaction(state, layout, cursor, viewport, renderer)
        }
    }
   
 
}
             
                
                
                
                
                
                
                
                
                
                
                
                
                            
            
                
                
                
                
                
            
                
                
                
                
                
                
                
                
                
            
                
                
                
                
                
            
                
            
                
                
                
                
                
