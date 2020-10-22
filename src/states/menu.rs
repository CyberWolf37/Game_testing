use amethyst::{
    ecs::prelude::Entity,
    core::transform::Transform,
    prelude::*,
    ui::{UiCreator, UiFinder},
};
 use log::{info};

pub const BUTTON_START: &str = "start";
pub const BUTTON_OPTIONS: &str = "options";
pub const BUTTON_CREDITS: &str = "credits";


#[derive(Default, Debug)]
pub struct Menu{
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for Menu {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;

        info!("Start");

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