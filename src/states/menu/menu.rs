use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct Menu{
}

impl SimpleState for Menu {

    fn on_start(_data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;

        world.register::<Items>();
        
    }
}

fn initialise_camera(world: &mut World){
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_enti
}