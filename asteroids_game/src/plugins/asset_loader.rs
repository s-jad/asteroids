use bevy::{
    app::{Plugin, Startup},
    asset::{AssetServer, Handle},
    ecs::system::{Res, ResMut, Resource},
    prelude,
    scene::Scene,
};

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid_1: Handle<Scene>,
    pub asteroid_2: Handle<Scene>,
    pub asteroid_4: Handle<Scene>,
    pub asteroid_3: Handle<Scene>,
    pub large_asteroid_1: Handle<Scene>,
    pub player_spaceship: Handle<Scene>,
    pub missile: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut prelude::App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid_1: asset_server.load("asteroid_1.glb#Scene0"),
        asteroid_2: asset_server.load("asteroid_2.glb#Scene0"),
        asteroid_3: asset_server.load("asteroid_3.glb#Scene0"),
        asteroid_4: asset_server.load("asteroid_4.glb#Scene0"),
        large_asteroid_1: asset_server.load("large_asteroid_1.glb#Scene0"),
        player_spaceship: asset_server.load("player_spaceship.glb#Scene0"),
        missile: asset_server.load("missile_1.glb#Scene0"),
    };
}
