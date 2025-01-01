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
    use std::borrow::{Borrow, BorrowMut};

    use iced::Theme;
    use iced_core::{layout::Node, mouse::Cursor, widget::Tree, Element, Length, Size, Vector, Widget};
    use iced_widget::{button, container, renderer::wgpu::{self, primitive::Renderer}, Button, Container};
 
    use crate::{Cell, CellMessage, Grid, GridMessage};

    fn construct_new_tree(tree: &iced_core::widget::Tree) -> iced_core::widget::Tree {
        let mut new_tree = iced_core::widget::Tree::empty(); // Start with an empty tree
    
        // Iterate through the original tree's children
        for (index, child) in tree.children.iter().enumerate() {
            if index > 0 {
                // Recursively construct a new tree for the child
                let new_child = construct_new_tree(child);
                new_tree.children.push(new_child);
            }
        }
    
        new_tree
    }
    

    pub fn style<'a, Message, Theme>
    (
        content: &'a mut Grid<Message, Theme>,
        theme: Theme,
        style: <Theme as super::Catalog>::Style,
    ) -> Element<'a, Message, Theme, iced_widget::Renderer>
    where
        
        Theme: super::Catalog<Themes = iced_core::Theme> + iced_widget::text::Catalog  + iced_widget::container::Catalog + Clone + 'a,
        Message: 'a,
        iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>
    {
        Element::new(
            Wrapper {
                content: Box::new(content),
                target: Style,
                theme,
                style: content.data.style.clone()
            }
        )
 
 
    }
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
    impl<'a, Message, Theme> Widget<Message, Theme, iced::Renderer>for Grid<Message, Theme>
    where
        Theme: super::Catalog<Themes = iced_core::Theme> + iced_widget::container::Catalog
        + Clone + iced_widget::text::Catalog,
        iced_core::Element<'a, Message, Theme, iced_widget::Renderer>: From<Grid<Message, Theme>>,
    {
            fn size(&self) -> Size<Length> {
               
               
               Size::new(Length::Fixed(self.data.width), Length::Fixed(self.data.height))
            }

            fn layout(
                &self,
                tree: &mut iced_core::widget::Tree,
                renderer: &iced::Renderer,
                limits: &iced_core::layout::Limits,
            ) -> iced_core::layout::Node {
                println!("B");
                let max_size = limits.max();
                let width = self.data.width.min(max_size.width);
                let height = self.data.height.min(max_size.height);
            
                let rows = self.data.rows.len();
                let cols = self.data.rows.get(0).map_or(0, |row| row.cells.len());
            
                if rows == 0 || cols == 0 {
                    return iced_core::layout::Node::new(iced_core::Size::ZERO);
                }
            
                let cell_width: f32 = 20.0;
                let cell_height: f32 = 20.0;
                let column_distance: f32 = 20.0;
                let row_distance: f32 = 20.0;
                

                println!("{}", tree.children.len());

                self.data.rows.iter().for_each(|row| {
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
                let new_container: Container<'_, Message, Theme, iced_widget::Renderer> = container("Test");
                tree.children = self.data
                .rows
                .iter()
                .flat_map(|row| {
                    row.cells.iter().map(|cell: &Cell<'_>| match cell {
                            Cell::Container(container) => {
                                iced_core::widget::Tree { tag: container.tag(), state: container.state(), children: container.children() }
                            },
                        _ => {
                            iced_core::widget::Tree { tag: new_container.tag(), state: new_container.state(), children: new_container.children() }
                        }
                    })
                })
                .collect();
                let mut children = Vec::new();
            
                for (row_index, row) in self.data.rows.iter().enumerate() {
                    if row.cells.is_empty() {
                        continue; // Skip empty rows
                    }
            
                    for (col_index, cell) in row.cells.iter().enumerate() {
                        let index = row_index * cols + col_index;
            
                        // Safely access tree.children[index]
                        println!("{}", tree.children.len());
                        println!("{}", index);
                        //if index < tree.children.len() {
                            let child_tree = &mut tree.children[index];
            
                            match cell {
                                Cell::Container(container) => {
                                    let position = iced_core::Point {
                                        x: col_index as f32 * (cell_width + column_distance),
                                        y: row_index as f32 * (cell_height + row_distance),
                                    };
            
                                    let size = iced_core::Size::new(cell_width, cell_height);
            
                                    let mut container_node = iced_core::layout::Node::new(size);
                                    children.push(container_node.move_to(position));
            
                                    println!(
                                        "Manually created container node at position: {:?} with size: {:?}",
                                        position, size
                                    );
                                    println!("Container state: {:?}", container.state());
                                    println!("Container tag: {:?}", container.tag());
                                }
                                _ => {
                                    let dummy_node = iced_core::layout::Node::new(iced_core::Size::new(cell_width, cell_height));
                                    let position = iced_core::Point {
                                        x: col_index as f32 * (cell_width + column_distance),
                                        y: row_index as f32 * (cell_height + row_distance),
                                    };
                                    let mut dummy_node = dummy_node.clone();
                                    children.push(dummy_node.move_to(position));
                                }
                            }
                        // } else {
                        //     println!("Index {} is out of bounds for tree.children", index);
                        // }
                    }
                }
                println!("Children: {}", children.len());
                iced_core::layout::Node::with_children(iced_core::Size::new(width, height), children)
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
                // Filter the layout to exclude the first Node
               // let filtered_layout = filter_layout(layout);
            
                let rows = self.data.rows.len();
                let cols = self.data.rows.get(0).map_or(0, |row| row.cells.len());
                println!("A");
      
                //traverse_tree(&tree);

            
                for (row_index, row) in self.data.rows.iter().enumerate() {
                    for (col_index, cell) in row.cells.iter().enumerate() {
                        let child_index = row_index * cols + col_index;
                        //println!("B");
                    if let Some(bounds) = layout
                        .children()
                        .nth(child_index)
                        .map(|child| child.bounds())
                    {
                            match cell {
                                Cell::Container(container) => {
                                    // println!("{:#?}", layout);
                                    // println!("{:#?}", tree);
                                    container.draw(
                                        tree,
                                        renderer,
                                        &theme.resolve_theme(),
                                        style,
                                        layout,
                                        cursor,
                                        viewport,
                                    );
                                    print!("Matched")
                                }
                                _ => {
                                    //print!("Not matched")
                                }
                            }
                        }
                    }
                }
            }
            // fn draw(
            //     &self,
            //     state: &Tree,
            //     renderer: &mut iced_widget::renderer::fallback::Renderer<Renderer,Renderer>,
            //     theme: &Theme,
            //     style: &iced_core::renderer::Style,
            //     layout: Layout<'_>,
            //     cursor: mouse::Cursor,
            //     viewport: &Rectangle,
            // ) {
            //     //let mut elements: Vec<Element<CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>>> = Vec::new();
            //     let mut elements: Vec<Element<CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>>> = Vec::new();

            //     for row in &self.rows {
            //         for cell in &row.cells {
            //             let element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_, _>> = match cell {
            //                 Cell::Container(container2) => {
            //                  //   let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = Element::new(*container2);
            //                  let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>> = Element::<CellMessage, Theme, iced_widget::renderer::fallback::Renderer<iced::Renderer, iced_tiny_skia::Renderer>
            //                  >::new(container("Es"));
            //                     return_element
            //                 },
            //                 _ => {
            //                     let newer_container: Container<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_, _>> = container("E");
            //                    // let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = Element::new(newer_container);
            //                    let return_element: Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<_,_>> = Element::new(newer_container);
            //                     return_element
            //                 },
            //             };
            //             elements.push(element);
            //         }
            //     }
                
            
            //     let new_container: Container<'_, Message, iced::Theme, iced_widget::renderer::fallback::Renderer<iced::Renderer, iced_tiny_skia::Renderer>> = container("Test");
            
                
            //     let new_states: Vec<_> = self
            //         .rows
            //         .iter()
            //         .flat_map(|row| {
            //             row.cells.iter().map(|cell: &Cell<'_>| match cell {
            //                 Cell::Container(container) => {
            //                     //container.state()
            //                     iced_core::widget::Tree { 
            //                         tag: container.tag(), 
            //                         state: container.state(), 
            //                         children: container.children() 
            //                     }
            //                 //    new_container.tag()
            //                  },
            //                 _ => {
            //                     // new_container.state()
            //                     iced_core::widget::Tree { 
            //                         tag: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::tag(&new_container), 
            //                         state: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::state(&new_container), 
            //                         children: <Container<'_, Message, _, iced_widget::renderer::fallback::Renderer<_,_>>>::children(&new_container) 
            //                     }
            //                 }
            //             })
            //         })
            //         .collect();
            
                
            //     // let new_root_state = Tree {
            //     //     tag: new_container.tag(), 
            //     //     state: new_container.state(), 
            //     //     children: new_states, 
            //     // };
            
                
            //     for ((element, state), l) in elements
            //         .iter()
            //         .zip(new_states.iter())  
            //         .zip(layout.children())
            //     {

            //         if l.children().next().is_none() {                        // println!("Skipping draw for layout: {:?}", l);
            //             continue;
            //         }
            //         println!("a");
            //         // println!("{:#?}", element.as_widget().layout(state, renderer, limits));
            //         println!("{:#?}", element.as_widget().state());
            //        // let new_renderer: &mut iced_widget::renderer::fallback::Renderer<iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>, iced_tiny_skia::Renderer> = renderer;
            //     //    let new_element: &Element<'_, CellMessage, _, iced_widget::renderer::fallback::Renderer<wgpu::Renderer, iced_tiny_skia::Renderer>> = element;

            //     //    new_element
            //     //         .as_widget()
            //     //         .draw(state, renderer, &iced::Theme::Dark, style, l, cursor, viewport);
            //         let test_element: Element<'_, _, Theme, iced_widget::renderer::fallback::Renderer<Renderer, Renderer>> = Element::<Message, Theme, iced_widget::renderer::fallback::Renderer<Renderer, Renderer>>::new(iced::widget::Text::new("E"));
            //         test_element.as_widget().draw(state, renderer, theme, style, layout, cursor, viewport);
            //     }
            // }
            

    }

    impl<'a, Inner, Theme, Renderer> Borrow<dyn iced_core::Widget<CellMessage, Theme, Renderer> + 'a>
    for Wrapper<'a, Inner, Theme>
        where
            Inner: crate::style::Catalog<Themes = iced::Theme>,
            Wrapper<'a, Inner, Theme>: Widget<CellMessage, Theme, Renderer>, // Ensure Wrapper implements Widget
            Renderer: iced_core::Renderer + 'a,
            Theme: crate::style::Catalog<Themes = iced_core::Theme> + 'a,
        {
            fn borrow(&self) -> &(dyn iced_core::Widget<CellMessage, Theme, Renderer> + 'a) {
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
             
                // tree.children = self.rows
                // .iter()
                // .flat_map(|row| {
                //     row.cells.iter().map(|cell: &Cell<'_>| match cell {
                //         Cell::Container(container) => {
                //             let cloned_container = container.clone();
                //             let wrapper = Wrapper {
                //                 content: cloned_container,
                //                 theme: self.theme.clone(),
                //                 style: self.style.clone(),
                //                 target: Style,
                //             };
                            
            
                //             let tree = iced_core::widget::Tree {
                //                 tag: wrapper.content.tag(),
                //                 state: wrapper.content.state(),
                //                 children: wrapper.content.children(),
                //             };
            
                //             tree // You're moving tree here
                //         },
                //         _ => {
                //             let new_wrapper = Wrapper {
                //                 content: &container("E"),
                //                 theme: self.theme.clone(),
                //                 style: self.style.clone(),
                //                 target: Style,
                //             };
            
                //             let mut tree = iced_core::widget::Tree {
                //                 tag: new_wrapper.tag(),
                //                 state: new_wrapper.state(),
                //                 children: new_wrapper.children(),
                //             };
            
                //             tree.diff(new_wrapper);
            
                //             tree // You're moving tree here
                //         }
                //     })
                // })
                // .collect();

                // fn traverse_tree(tree: &iced_core::widget::Tree) {
                //     // Perform operations on the current node
                //     // For example, print the tag of the node
                //     println!("start");
                //     println!("{:?}", tree.state);
                //     println!("{:?}", tree.tag);
                //     println!("end");
                //     // Recursively traverse child nodes
                //     for child in &tree.children {
                //         traverse_tree(child);
                //     }
                // }
                 
    // fn filter_layout(layout: iced_core::Layout<'_>) -> iced_core::layout::Node {
    //     // Collect all children except the first, converting Layout to Node
    //     let filtered_children: Vec<iced_core::layout::Node> = layout
    //         .children()
    //         .enumerate()
    //         .filter_map(|(index, child)| {
    //             if index > 0 {
    //                 Some(iced_core::layout::Node::new(child.bounds().size())) // Convert Layout to Node
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();
    
    //     // Construct a new Node with the filtered children
    //     iced_core::layout::Node::with_children(
    //         layout.bounds().size(),
    //         filtered_children, // Pass the filtered children
    //     )
    // }