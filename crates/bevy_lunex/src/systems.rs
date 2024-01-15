use std::marker::PhantomData;
use bevy::prelude::*;
use lunex_engine::*;

/// ## Compute camera ui
/// This function triggers computation method on camera's [`UINodeTree`] component if there was a change.
pub fn compute_camera_ui<T:Default + Component>(mut query: Query<(&Camera, &mut UINodeTree<T>), Or<(Changed<Camera>, Changed<UINodeTree<T>>)>>) {
    for (cam, mut ui) in &mut query {

        // Extract camera size
        if let Some(size) = cam.physical_viewport_size() {
            let size: (u32, u32) = size.into();

            // Compute the UI with the extracted size
            ui.compute(Rect2D::new().with_size((size.0 as f32, size.1 as f32)).into());
        }
    }
}




/// ## UI plugin
/// THis
#[derive(Debug, Default, Clone)]
pub struct UIPlugin <T:Default + Component>(pub PhantomData<T>);
impl <T:Default + Component> UIPlugin<T> {
    pub fn new() -> Self {
        UIPlugin::<T>(PhantomData)
    }
}
impl <T:Default + Component> Plugin for UIPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, compute_camera_ui::<T>);
    }
}