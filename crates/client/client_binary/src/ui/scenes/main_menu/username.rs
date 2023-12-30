use bevy::{
    app::{Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::{With, Without},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Query, Res, ResMut, Resource, SystemId},
    },
    hierarchy::{BuildChildren, Children, HierarchyQueryExt},
    prelude::default,
    reflect::TypePath,
    text::{Text, TextAlignment, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, JustifyContent, PositionType, Style, UiRect, Val,
    },
    utils::Uuid,
};
use bevy_simple_text_input::{TextInput, TextInputInner};
use core_library::{
    async_runners,
    authentication::{
        client_authentication::ClientAuthenticationInfo, AppAuthenticationState,
        AuthenticationServerInfo,
    },
    network::HttpRequestMeta,
    player::{GetPlayerUsernameResponse, SetPlayerUsernameRequest, SetPlayerUsernameResponse},
    AsyncChannel, TaskPoolRes,
};
use ehttp::Response;

use crate::ui::{
    colors::CurrentColors,
    marker_component,
    scenes::cleanup_scene,
    widgets::{
        basic_button::{basic_button, BasicButtonAppExtension, ButtonStyle},
        modal::{modal_panel, ModalStyle},
    },
};
pub struct UsernameUiPlugin;

impl Plugin for UsernameUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let setup = app.world.register_system(setup_set_username);
        let cleanup = app.world.register_system(cleanup_scene::<UsernameModal>);

        app.insert_resource(UsernameSystems { setup, cleanup });

        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            check_username,
        );
        app.add_systems(
            Update,
            (
                handle_check_username_responses,
                handle_set_username_responses,
                update_intermediate_username_and_error_text,
            )
                .run_if(in_state(AppAuthenticationState::Authenticated)),
        );

        app.insert_resource(AsyncChannel::<CheckUsernameResponse>::default());
        app.insert_resource(AsyncChannel::<SetUsernameResponse>::default());

        app.add_button_handler(marker_component::<UsernameButton>(), set_username);
        app.insert_resource(IntermediateUsernameResource {
            username: String::default(),
            error_text: String::default(),
        });
    }
}

#[derive(Resource)]
struct UsernameSystems {
    setup: SystemId,
    cleanup: SystemId,
}

#[derive(Clone)]
struct CheckUsernameResponse {
    response: Result<Response, String>,
}

#[derive(Clone)]
struct SetUsernameResponse {
    response: Result<Response, String>,
}

#[derive(Component, TypePath)]
struct UsernameModal;

#[derive(Component, TypePath)]
struct UsernameEntity;

#[derive(Component, TypePath)]
struct UsernameButton;

#[derive(Component, TypePath)]
struct UsernameErrorText;

/// A resource storing intermediate information for username related things
#[derive(Resource)]
struct IntermediateUsernameResource {
    username: String,
    error_text: String,
}

#[derive(Resource)]
struct PlayerHasCustomUsername;

fn check_username(
    client_info: Res<ClientAuthenticationInfo>,
    auth_server_info: Res<AuthenticationServerInfo>,
    task_pool_res: Res<TaskPoolRes>,
    check_username_channel: ResMut<AsyncChannel<CheckUsernameResponse>>,
) {
    let addr = auth_server_info.addr.clone();

    let check_username_channel = check_username_channel.clone();
    let client_info = client_info.clone();

    if let Some(task) = async_runners::run_async(
        async move {
            let mut request = ehttp::Request::get(format!("{}player/get_username", addr));

            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());
            request.headers.insert(
                "authorization".to_string(),
                format!("Bearer {}", client_info.sign_in_info.access_token.clone()),
            );
            match ehttp::fetch_async(request).await {
                Ok(response) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(CheckUsernameResponse {
                            response: Ok(response),
                        });
                }
                Err(err) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(CheckUsernameResponse { response: Err(err) });
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
}

fn handle_check_username_responses(
    check_username_responses: ResMut<AsyncChannel<CheckUsernameResponse>>,
    mut commands: Commands,
    systems: Res<UsernameSystems>,
) {
    if let Ok(receiver) = check_username_responses.reciever_channel.try_lock() {
        if let Ok(message) = receiver.try_recv() {
            if let Ok(message) = message.response {
                if message.status != 200 {
                    println!("Error: {}", message.status_text);
                    return;
                }

                let result_text = match message.text() {
                    Some(body) => body,
                    None => {
                        println!("No body");
                        return;
                    }
                };
                let username = match serde_json::from_str::<GetPlayerUsernameResponse>(result_text)
                {
                    Ok(result) => result,
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                        return;
                    }
                };

                match Uuid::parse_str(&username.username) {
                    Ok(_) => commands.run_system(systems.setup),
                    Err(_) => {
                        commands.insert_resource(PlayerHasCustomUsername);
                    }
                }
            };
        }
    }
}

fn setup_set_username(
    mut commands: Commands,
    colors: Res<CurrentColors>,
    has_custom_username: Option<Res<PlayerHasCustomUsername>>,
) {
    let can_close = has_custom_username.is_some();

    let modal = modal_panel(
        UsernameModal,
        ModalStyle {
            with_close_button: can_close,
            close_button_bundle: None::<()>,
            modal_size: Some((Val::Percent(75.0), Val::Percent(75.0))),
            outline: true,
        },
        &colors,
        &mut commands,
    );

    commands.entity(modal).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Arts Game",
                TextStyle {
                    font_size: 60.0,
                    color: colors.dark_text(),
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
    });

    let username_input = commands
        .spawn((
            NodeBundle {
                style: Style {
                    min_width: Val::Px(300.0),
                    width: Val::Px(300.0),
                    border: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    position_type: PositionType::Relative,
                    ..default()
                },
                border_color: colors.accent().into(),
                background_color: colors.background_dark().into(),
                ..default()
            },
            TextInput {
                text_style: TextStyle {
                    font_size: 24.,
                    color: colors.light_text(),
                    ..default()
                },
                inactive: true,
            },
            UsernameEntity,
        ))
        .id();

    let set_username_button = basic_button(
        UsernameButton,
        ButtonStyle {
            bundle: None::<()>,
            text: String::from("Set Username"),
            font_size: 40.0,
        },
        &mut commands,
        &colors,
    );

    let notifs = commands
        .spawn((
            TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 25.0,
                    color: colors.dark_text(),
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
            UsernameErrorText,
        ))
        .id();

    commands
        .entity(modal)
        .push_children(&[username_input, set_username_button, notifs]);
}

fn set_username(
    username: Option<ResMut<IntermediateUsernameResource>>,
    client_info: Res<ClientAuthenticationInfo>,
    auth_server_info: Res<AuthenticationServerInfo>,
    task_pool_res: Res<TaskPoolRes>,
    check_username_channel: ResMut<AsyncChannel<SetUsernameResponse>>,
) {
    let Some(mut username) = username else {
        return;
    };

    username.error_text.clear();

    let addr = auth_server_info.addr.clone();

    let check_username_channel = check_username_channel.clone();
    let client_info = client_info.clone();

    let message = match serde_json::to_string(&HttpRequestMeta {
        request: SetPlayerUsernameRequest {
            username: username.username.clone(),
        },
    }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(_) => {
            return;
        }
    };

    if let Some(task) = async_runners::run_async(
        async move {
            let mut request = ehttp::Request::post(format!("{}player/set_username", addr), message);

            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());
            request.headers.insert(
                "authorization".to_string(),
                format!("Bearer {}", client_info.sign_in_info.access_token.clone()),
            );
            match ehttp::fetch_async(request).await {
                Ok(response) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(SetUsernameResponse {
                            response: Ok(response),
                        });
                }
                Err(err) => {
                    let _ = check_username_channel
                        .sender_channel
                        .send(SetUsernameResponse { response: Err(err) });
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
}

fn handle_set_username_responses(
    check_username_responses: ResMut<AsyncChannel<SetUsernameResponse>>,
    mut username_resource: ResMut<IntermediateUsernameResource>,
    mut commands: Commands,
    systems: Res<UsernameSystems>,
) {
    if let Ok(receiver) = check_username_responses.reciever_channel.try_lock() {
        if let Ok(message) = receiver.try_recv() {
            if let Ok(message) = message.response {
                if message.status != 200 {
                    println!("Error: {}", message.status_text);
                    return;
                }

                let result_text = match message.text() {
                    Some(body) => body,
                    None => {
                        println!("No body");
                        return;
                    }
                };

                match serde_json::from_str::<SetPlayerUsernameResponse>(result_text) {
                    Ok(result) => match result {
                        SetPlayerUsernameResponse::Ok { new_username: _ } => {
                            commands.insert_resource(PlayerHasCustomUsername);
                            commands.run_system(systems.cleanup);
                            commands.run_system(systems.setup);
                        }
                        SetPlayerUsernameResponse::Error { error_text } => {
                            username_resource.error_text = error_text
                        }
                    },
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                    }
                };
            };
        }
    }
}

fn update_intermediate_username_and_error_text(
    username: Query<Entity, With<UsernameEntity>>,
    mut error_text: Query<(Entity, &mut Text), (With<UsernameErrorText>, Without<TextInputInner>)>,
    mut text_query: Query<&mut Text, With<TextInputInner>>,
    children_query: Query<&Children>,
    mut username_resource: ResMut<IntermediateUsernameResource>,
) {
    let Ok(username_entity) = username.get_single() else {
        return;
    };

    let Ok((_, mut text)) = error_text.get_single_mut() else {
        return;
    };
    let mut username = String::default();
    for descendant in children_query.iter_descendants(username_entity) {
        if let Ok(text) = text_query.get_mut(descendant) {
            username = format!("{}{}", text.sections[0].value, text.sections[2].value);
        } else {
            continue;
        }
    }

    if username_resource.is_changed() {
        text.sections[0].value = username_resource.error_text.clone();
    }

    username_resource.username = username;
}
