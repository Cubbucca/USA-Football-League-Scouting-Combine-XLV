use crate::{assets::GameAssets, game_state, ingame, splash, title_screen, AppState};
use bevy::{asset::Asset, ecs::system::SystemParam, gltf::Gltf, prelude::*};
use bevy_kira_audio::AudioSource;
use std::marker::PhantomData;

pub struct AssetLoadingPlugin;
impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NextState>()
            .init_resource::<AssetsLoading>()
            .add_system_set(
                SystemSet::on_update(AppState::Loading).with_system(check_assets_ready),
            );
    }
}

#[derive(Default)]
pub struct GameTexture {
    pub material: Handle<StandardMaterial>,
    pub image: Handle<Image>,
}

pub struct NextState {
    state: AppState,
}
impl Default for NextState {
    fn default() -> Self {
        NextState {
            state: AppState::TitleScreen,
        }
    }
}

#[derive(Default)]
pub struct AssetsLoading {
    pub asset_handles: Vec<HandleUntyped>,
}

#[derive(SystemParam)]
pub struct AssetsHandler<'w, 's> {
    asset_server: Res<'w, AssetServer>,
    assets_loading: ResMut<'w, AssetsLoading>,
    meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub images: ResMut<'w, Assets<Image>>,
    state: ResMut<'w, State<AppState>>,
    next_state: ResMut<'w, NextState>,

    #[system_param(ignore)]
    phantom: PhantomData<&'s ()>,
}

impl<'w, 's> AssetsHandler<'w, 's> {
    fn add_asset<T: Asset>(&mut self, asset: &mut Handle<T>, path: &str) {
        *asset = self.asset_server.load(path);
        self.assets_loading
            .asset_handles
            .push(asset.clone_untyped());
    }

    pub fn load(
        &mut self,
        next_state: AppState,
        game_assets: &mut ResMut<GameAssets>,
        game_state: &ResMut<game_state::GameState>,
    ) {
        self.queue_assets_for_state(&next_state, game_assets, game_state);
        self.next_state.state = next_state;
        self.state.set(AppState::Loading).unwrap();
    }

    pub fn add_mesh(&mut self, mesh: &mut Handle<Mesh>, path: &str) {
        self.add_asset(mesh, path);
    }

    pub fn add_font(&mut self, font: &mut Handle<Font>, path: &str) {
        self.add_asset(font, path);
    }

    pub fn add_audio(&mut self, audio: &mut Handle<AudioSource>, path: &str) {
        self.add_asset(audio, path);
    }

    pub fn add_glb(&mut self, glb: &mut Handle<Gltf>, path: &str) {
        self.add_asset(glb, path);
    }

    pub fn add_animation(&mut self, animation: &mut Handle<AnimationClip>, path: &str) {
        self.add_asset(animation, path);
    }

    pub fn add_standard_mesh(&mut self, handle: &mut Handle<Mesh>, mesh: Mesh) {
        *handle = self.meshes.add(mesh);
    }

    pub fn add_standard_material(
        &mut self,
        handle: &mut Handle<StandardMaterial>,
        material: StandardMaterial,
    ) {
        *handle = self.materials.add(material);
    }

    pub fn add_material(&mut self, game_texture: &mut GameTexture, path: &str, transparent: bool) {
        self.add_asset(&mut game_texture.image, path);
        game_texture.material = self.materials.add(StandardMaterial {
            base_color_texture: Some(game_texture.image.clone()),
            alpha_mode: if transparent {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        });
    }

    fn queue_assets_for_state(
        &mut self,
        state: &AppState,
        game_assets: &mut ResMut<GameAssets>,
        game_state: &ResMut<game_state::GameState>,
    ) {
        match state {
            AppState::Splash => splash::load(self, game_assets),
            AppState::TitleScreen => title_screen::load(self, game_assets),
            AppState::InGame => ingame::load(self, game_assets, game_state),
            _ => (),
        }
    }
}

fn check_assets_ready(mut assets_handler: AssetsHandler) {
    use bevy::asset::LoadState;

    let mut ready = true;
    for handle in assets_handler.assets_loading.asset_handles.iter() {
        match assets_handler.asset_server.get_load_state(handle) {
            LoadState::Failed => {
                println!("An asset had an error: {:?}", handle);
            }
            LoadState::Loaded => {}
            _ => {
                ready = false;
            }
        }
    }

    if ready {
        assets_handler.assets_loading.asset_handles = vec![]; // clear list since we've loaded everything
        assets_handler
            .state
            .set(assets_handler.next_state.state)
            .unwrap(); // move to next state
    }
}
