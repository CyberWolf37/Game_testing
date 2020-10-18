use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const BUTTON_START: &str = "start";
pub const BUTTON_LOAD: &str = "load";
pub const BUTTON_OPTIONS: &str = "options";
pub const BUTTON_CREDITS: &str = "credits";


pub struct Menu{
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for Menu {

    fn on_start(_data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;

        self.ui_root =
        Some(world.exec(|mut creator: UiCreator<'_>| creator.create("config/ui/menu.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
            });
        }

        Trans::None
    }
}

impl Default for Menu {
    fn default() -> Self {
        Menu {

        }
    }
}