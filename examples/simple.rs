use bevy::prelude::*;
use ron_asset_manager::prelude::*;
use serde::Deserialize;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RonAssetPlugin::<Car>::default())
        .init_resource::<GameAssets>()
        .add_systems(Startup, setup)
        .add_systems(Update, loaded)
        .run();
}

#[derive(Resource, Default)]
pub struct GameAssets {
    pub car: Handle<Car>,
}

fn setup(server: Res<AssetServer>, mut assets: ResMut<GameAssets>) {
    assets.car = server.load("my_car.ron");
}

fn loaded(game_assets: Res<GameAssets>, car_assets: Res<Assets<Car>>) {
    let Some(car) = car_assets.get(&game_assets.car) else {
        return;
    };

    dbg!(car);
}

#[derive(Asset, RonAsset, TypePath, Deserialize, Debug)]
pub struct Car {
    pub speed: f32,
    pub name: String,
    #[asset]
    pub body_sprite: Shandle<Image>,
    #[asset]
    pub wheels: Vec<Wheel>,
}

#[derive(RonAsset, Deserialize, Default, Debug)]
pub struct Wheel {
    #[asset]
    pub sprite: Shandle<Image>,
    pub position: Vec2,
    pub can_turn: bool,
}
