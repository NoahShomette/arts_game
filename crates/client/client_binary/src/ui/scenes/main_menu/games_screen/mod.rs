//! The games menu does three things
//! - Show a player all games they are in
//! - Show a player all public open games they can join
//! - Create a new game
//!
//! The first two options should basically be tabs

use bevy::{
    app::{Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Query, Res, Resource},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt},
    prelude::default,
    reflect::TypePath,
    render::color::Color,
    text::{TextAlignment, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, UiRect, Val,
    },
    utils::{HashMap, Instant},
};
use core_library::{
    auth_server::game::GameAuthServerInfo, game_meta::GameId, network::game_http::GameMetaInfo,
};

use crate::ui::{
    colors::CurrentColors,
    marker_component,
    scenes::ScenesAppExtension,
    widgets::tabbed_content::{tabbed_content, TabbedContentSettings},
};

use self::{
    game_entry_button::{GameEntryButtonPlugin, GameEntryButtonStyle},
    games_data::GamesDataPlugin,
};
use super::MainMenuScreenState;
use crate::ui::scenes::main_menu::games_screen::game_entry_button::game_entry_button;

mod game_entry_button;
mod games_data;

pub struct GamesMenuPlugin;

impl Plugin for GamesMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((GameEntryButtonPlugin, GamesDataPlugin));
        app.add_scene(
            marker_component::<GamesScreenRootMarker>(),
            setup_games,
            MainMenuScreenState::GamesPage,
        );

        app.add_systems(
            Update,
            (update_open_games_screen,).run_if(in_state(MainMenuScreenState::GamesPage)),
        );
    }
}

#[derive(Resource)]
pub struct OpenGamesData {
    pub games: HashMap<
        GameId,
        (
            GameAuthServerInfo,
            Option<GameMetaInfo>,
            Option<LastRequestInfo>,
        ),
    >,
}

#[derive(Resource)]
pub struct PlayerGamesData {
    pub games: HashMap<
        GameId,
        (
            GameAuthServerInfo,
            Option<GameMetaInfo>,
            Option<LastRequestInfo>,
        ),
    >,
}

pub struct LastRequestInfo {
    pub request_time: Instant,
}

#[derive(Component)]
pub struct GamesTabs;

#[derive(Component, TypePath)]
struct GamesScreenRootMarker;

#[derive(Component, TypePath)]
struct PlayerGamesButtonsHolder;

#[derive(Component, TypePath)]
struct OpenGamesButtonsHolder;

#[derive(Component, TypePath)]
struct GameEntryMarker;

fn setup_games(mut commands: Commands, colors: Res<CurrentColors>) {
    // root ui for entire screen
    let screen_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(GamesScreenRootMarker)
        .id();

    // top area ui
    let top_ui = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Games",
                    TextStyle {
                        font_size: 100.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(75.0)),
                    ..default()
                }),
            );
        })
        .id();

    let (tab_root, tabs) = tabbed_content(
        GamesTabs,
        TabbedContentSettings {
            tabs: vec!["Games".to_string(), "Open Games".to_string()],
            open_tab: 0,
        },
        &colors,
        &mut commands,
    );

    for (i, (entity, _)) in tabs.iter().enumerate() {
        if i == 0 {
            commands.entity(*entity).insert(PlayerGamesButtonsHolder);
        }
        if i == 1 {
            commands.entity(*entity).insert(OpenGamesButtonsHolder);
        }
    }

    commands
        .entity(screen_container)
        .push_children(&[top_ui, tab_root]);
}

fn update_open_games_screen(
    mut commands: Commands,
    colors: Res<CurrentColors>,
    open_games_query: Query<(Entity, &mut Children), With<OpenGamesButtonsHolder>>,
    open_games: Res<OpenGamesData>,
) {
    if open_games.is_changed() {
        let (parent_entity, children) = open_games_query.single();
        for child in children.iter() {
            commands.entity(*child).despawn_recursive();
        }
        let mut tab_entities = vec![];
        for (_, game) in open_games.games.iter() {
            let Some(full_info) = &game.1 else {
                continue;
            };
            let tab_content = game_entry_button(
                GameEntryMarker,
                GameEntryButtonStyle {
                    game_name: full_info.game_name.clone(),
                    player_count: full_info.game_players.count(),
                    max_player_count: full_info.max_players,
                    server_type: game.0.server_type.clone(),
                    started_time: full_info.game_start_time,
                },
                &mut commands,
                &colors,
            );
            tab_entities.push(tab_content);
        }
        commands.entity(parent_entity).push_children(&tab_entities);
    }
}
