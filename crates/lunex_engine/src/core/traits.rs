use std::borrow::Borrow;

use bevy::ecs::component::Component;
use bevy::log::info;
use bevy::math::Vec3Swizzles;

use crate::nodes::prelude::*;
use crate::layout;
use crate::Layout;
use crate::MasterData;
use crate::Rect2D;
use crate::Rect3D;
use crate::import::*;

use super::{UiNode, UiTree, NodeData};




// #============================#
// #=== DIRECT UINODE TRAITS ===#

/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeCreationTrait<N:Default + Component> {
    /// ## Make node
    /// Makes new subnode in this node and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::create_node`] for hierarchy creation
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Create node
    /// Creates new subnode in this node or any other subnode and returns the new subnodes' name.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::make_node`] for direct creation
    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>;

    /// ## Obtain or create node
    /// Borrows subnode from this node. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node`] for hierarchy retrieval
    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Obtain or create node mut
    /// Borrows subnode from this node as mut. If the node doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::borrow_or_create_node_mut`] for hierarchy retrieval
    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;

    /// ## Borrow or create node
    /// Borrows subnode from this node or any other subnode. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node`] for direct retrieval
    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError>;

    /// ## Borrow or create node mut
    /// Borrows subnode from this node or any other subnode as mut. If a node in path doesn't exist, it creates one.
    /// ### 📌 Note
    /// * Use [`NodeCreationTrait::obtain_or_create_node_mut`] for direct retrieval
    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError>;  
}
impl <M: Default + Component, N: Default + Component> UiNodeCreationTrait<N> for UiTree<M, N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.make_ui_node(name)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError>{
        self.node.create_ui_node(path)
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.obtain_or_create_ui_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node(path)
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        self.node.borrow_or_create_ui_node_mut(path)
    }
}
impl <N: Default + Component> UiNodeCreationTrait<N> for UiNode<N> {
    fn make_ui_node(&mut self, name: impl Borrow<str>) -> Result<String, NodeError> {
        let n = self.make_node(name)?;
        self.insert_data(n.clone(), NodeData::default())?;
        Ok(n)
    }

    fn create_ui_node(&mut self, path: impl Borrow<str>) -> Result<String, NodeError> {
        let mut node: UiNode<N> = Node::new();
        node.add_data(NodeData::default());
        self.insert_node(path, Node::new())
    }

    fn obtain_or_create_ui_node(&mut self, name: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node(n)
        }
        self.obtain_node(name)
    }

    fn obtain_or_create_ui_node_mut(&mut self, name: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        if let Ok(n) = self.make_ui_node(name.borrow()) {
            return self.obtain_node_mut(n)
        }
        self.obtain_node_mut(name)
    }

    fn borrow_or_create_ui_node(&mut self, path: impl Borrow<str>) -> Result<&UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node(rempath),
        }
    }

    fn borrow_or_create_ui_node_mut(&mut self, path: impl Borrow<str>) -> Result<&mut UiNode<N>, NodeError> {
        match path.borrow().split_once('/') {
            None => self.obtain_or_create_ui_node_mut(path),
            Some((name, rempath)) => self.obtain_or_create_ui_node_mut(name)?.borrow_or_create_ui_node_mut(rempath),
        }
    }
}

/// ## UiNode data trait
/// Trait that abstracts over [`NodeDataTrait`] to provide tailored
/// implementations for [`UiTree`] data management.
pub trait UiNodeDataTrait<N> {
    /// ## Add ui data
    /// Adds new data to this node and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::insert_ui_data`] for hierarchy insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn add_ui_data(&mut self, data: N) -> Option<N>;

    /// ## Insert ui data
    /// Inserts new data to this node or any other subnode and returns the previous data.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::add_ui_data`] for direct insert
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError>;

    /// ## Take ui data
    /// Removes data from this node and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::remove_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn take_ui_data(&mut self) -> Option<N>;

    /// ## Remove ui data
    /// Removes data from this node or any other subnode and returns them.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::take_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError>;

    /// ## Obtain ui data
    /// Borrows data from this node.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data(&self) -> Option<&N>;

    /// ## Obtain ui data mut
    /// Borrows data from this node as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::borrow_ui_data_mut`] for hierarchy retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn obtain_ui_data_mut(&mut self) -> Option<&mut N>;

    /// ## Borrow ui data
    /// Borrows data from this node or any other subnode.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError>;

    /// ## Borrow ui data mut
    /// Borrows data from this node or any other subnode as mut.
    /// ### 📌 Note
    /// * Use [`UiNodeDataTrait::obtain_ui_data_mut`] for direct retrieval
    /// ### ⚠️ Panics
    /// * Panics if [`UiNode`] is missing [`NodeData`] data _(should not happen unless you used methods not in prelude)_.
    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError>;
}
impl <M: Default + Component, N: Default + Component> UiNodeDataTrait<N> for UiTree<M, N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        self.node.add_ui_data(data)
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        self.node.insert_ui_data(path, data)
    }

    fn take_ui_data(&mut self) -> Option<N> {
        self.node.take_ui_data()
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        self.node.remove_ui_data(path)
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        self.node.obtain_ui_data()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        self.node.obtain_ui_data_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        self.node.borrow_ui_data(path)
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        self.node.borrow_ui_data_mut(path)
    }
}
impl <N: Default + Component> UiNodeDataTrait<N> for UiNode<N> {
    fn add_ui_data(&mut self, data: N) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, Some(data))
    }

    fn insert_ui_data(&mut self, path: impl Borrow<str>, data: N) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, Some(data)))
    }

    fn take_ui_data(&mut self) -> Option<N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        core::mem::replace(&mut container.data, None)
    }

    fn remove_ui_data(&mut self, path: impl Borrow<str>) -> Result<Option<N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(core::mem::replace(&mut container.data, None))
    }

    fn obtain_ui_data(&self) -> Option<&N> {
        let Some(container) = self.obtain_data() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_ref()
    }

    fn obtain_ui_data_mut(&mut self) -> Option<&mut N> {
        let Some(container) = self.obtain_data_mut() else { panic!("This UiNode is missing Ui data!") };
        container.data.as_mut()
    }

    fn borrow_ui_data(&self, path: impl Borrow<str>) -> Result<Option<&N>, NodeError> {
        let Some(container) = self.borrow_data(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_ref())
    }

    fn borrow_ui_data_mut(&mut self, path: impl Borrow<str>) -> Result<Option<&mut N>, NodeError> {
        let Some(container) = self.borrow_data_mut(path)? else { panic!("This UiNode is missing Ui data!") };
        Ok(container.data.as_mut())
    }
}


/// ## UiNodetree init trait
/// Trait that abstracts over [`NodeTreeInitTrait`] to provide tailored
/// implementations for [`UiTree`] initialization.
pub trait UiNodeTreeInitTrait {
    /// ## New
    /// Creates new UiTree.
    fn new(name: impl Borrow<str>) -> Self;
}
impl <M: Default + Component, N: Default + Component> UiNodeTreeInitTrait for UiTree<M, N> {
    fn new(name: impl Borrow<str>) -> Self {
        let mut tree: UiTree<M, N> = NodeTreeInitTrait::new(name);
        tree.add_topdata(MasterData::default());
        tree.add_data(NodeData::default());
        tree
    }
}



/// ## Node tree compute trait
/// Trait with all node tree layout computation implementations.
pub trait UiNodeTreeComputeTrait {
    fn compute(&mut self, parent: Rect3D);
}
impl <M: Default + Component, N: Default + Component> UiNodeTreeComputeTrait for UiTree<M, N> {
    fn compute(&mut self, parent: Rect3D) {

        let mut abs_scale = 1.0;
        let mut font_size = 16.0;

        if let Some(master_data) = self.obtain_topdata() {
            abs_scale = master_data.abs_scale;
            font_size = master_data.font_size;
        }

        self.node.compute_layout(parent, abs_scale, font_size);
        self.node.compute(abs_scale, font_size);
    }
}

/// ## Node compute trait
/// Trait with all node layout computation implementations. Includes private methods.
trait UiNodeComputeTrait {
    fn compute(&mut self, abs_scale: f32, font_size: f32);
    fn compute_layout(&mut self, parent: Rect3D, abs_scale: f32, font_size: f32) -> bool;
    fn compute_content(&mut self, computed: bool, position: Vec2, size: Vec2, abs_scale: f32, font_size: f32) -> Vec2;
}
impl <N:Default + Component> UiNodeComputeTrait for UiNode<N> { 
    fn compute_layout(&mut self, parent: Rect3D, abs_scale: f32, mut font_size: f32) -> bool {

        let mut computed = false;

        // Get depth before mutating self
        let depth = self.get_depth();
        
        // Check here if computation is required for partial recalculation
        if let Some(node_data) = &mut self.data {

            // Overwrite passed style with font size
            if let Some(fnt) = node_data.font_size { font_size = fnt }

            // Compute node layout
            match &node_data.layout {
                Layout::Window(l) => node_data.rectangle = l.compute(parent.into(), abs_scale, font_size).into(),
                Layout::Solid(l)  => node_data.rectangle = l.compute(parent.into(), abs_scale, font_size).into(),
                _ => { computed = true },
            }

            // Assing depth
            node_data.rectangle.pos.z = depth;
        }
        computed
    }
    fn compute(&mut self, abs_scale: f32, font_size: f32) {

        if let Some(ancestor_data) = &mut self.data {

            // Enter recursion
            for (_, subnode) in &mut self.nodes {
                let computed = subnode.compute_layout(ancestor_data.rectangle, abs_scale, font_size);

                if let Some(node_data) = &subnode.data {
                    subnode.compute_content(computed, node_data.rectangle.pos.xy(), node_data.rectangle.size, abs_scale, font_size);
                }

                subnode.compute(abs_scale, font_size);
            }
        }

    }
    fn compute_content(&mut self, computed: bool, position: Vec2, size: Vec2, abs_scale: f32, font_size: f32) -> Vec2 {

        if computed == true { return self.data.as_ref().unwrap().content_size; }

        let mut matrix: Vec<Vec<&mut Node<NodeData<N>>>> = Vec::new();
        let mut content_size = Vec2::ZERO;

        // Sort mutable pointers into matrix
        let mut i = 0;
        matrix.push(Vec::new());
        for (_, subnode) in &mut self.nodes {
            if let Some(subnode_data) = &subnode.data {
                if let Layout::Div(layout) = &subnode_data.layout {
                    let br = layout.force_break;
                    matrix[i].push(subnode);
                    if br {
                        i += 1;
                        matrix.push(Vec::new());
                    }
                }
            }
        }

        // Get the offset position
        //let mut line_offset = 0.0;
        let mut cursor = Vec2::ZERO;

        // Loop over each line in matrix to calculate position
        for line in &mut matrix {

            // Loop over each subnode in line to calculate position
            cursor.x = 0.0;
            let mut previous_margin_x = 0.0;
            let mut line_height = 0.0;
            for subnode in line {

                // Compute padding
                let padding = if let Layout::Div(layout) = &subnode.data.as_ref().unwrap().layout { layout.compute_padding(size, abs_scale, font_size) } else { unreachable!(); };
                let pos = Vec2::new(position.x + padding.x, position.y + padding.y);
                //info!("{}", pos);
                //let pos = Vec2::ZERO;
                let potential_content = subnode.compute_content(false, pos, size, abs_scale, font_size);

                // Unwrap guaranteed data
                let subnode_data = subnode.data.as_mut().unwrap();
                if let Layout::Div(layout) = &subnode_data.layout {
                    
                    // Compute size
                    let mut subnode_content = subnode_data.content_size;
                    if potential_content != Vec2::ZERO { subnode_content = potential_content }
                    let (size, margin) = layout.compute(subnode_content, size, abs_scale, font_size);



                    // Apply primary margin
                    cursor.x += f32::max(previous_margin_x, margin.x);

                    // Construct with primary margin
                    subnode_data.rectangle = Rect2D {
                        pos: Vec2 {
                            x: position.x + cursor.x,
                            y: position.y + cursor.y + margin.y,
                        },
                        size,
                    }.into();

                    // Apply secondary margin
                    cursor.x += size.x;

                    line_height = f32::max(line_height, margin.y + size.y + margin.w);
                    previous_margin_x = margin.z;
                }
            }
            cursor.y += line_height;
            content_size.x = f32::max(content_size.x, cursor.x);
        }
        content_size.y = cursor.y;
        content_size
    }
}


// #========================================#
// #=== FUNCTIONALITY WITH UINODE TRAITS ===#


/// ## Build as node
/// Trait that [Layout] types implement so they can be build as new node.
pub trait BuildAsNode {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized;
}
impl BuildAsNode for layout::Window {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}
impl BuildAsNode for layout::Solid {
    fn build<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>) -> Result<String, NodeError> where Self: Sized {
        ui.create_node(path.borrow())?;
        let mut container: NodeData<N> = NodeData::new();
        container.layout = self.into();
        ui.insert_data(path, container)?;
        Ok(String::new())
    }
}


/// ## Sync to node
/// Trait that [Component] types which represent values in [UiTree] need to
/// implement to load and store data in [UiTree].
pub trait SyncToNode {
    fn load<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
    fn save<M: Default + Component, N: Default + Component>(self, ui: &mut UiTree<M, N>, path: impl Borrow<str>);
}





pub trait Extract <T> {
    fn get_extract (&self) -> T;
    fn set_extract (&mut self, val: T) -> T;
}