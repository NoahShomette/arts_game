use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children, HierarchyQueryExt},
    prelude::default,
    reflect::TypePath,
    text::{Text, TextAlignment, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, JustifyContent, PositionType, Style, UiRect, Val,
    },
};
use bevy_simple_text_input::{TextInput, TextInputInner};
use core_library::authentication::{
    client_authentication::{PasswordLoginInfo, SignInEvent, SignUpEvent},
    SignUpResultEvent,
};

use crate::ui::{
    colors::CurrentColors,
    widgets::{
        basic_button::{basic_button, ButtonStyle},
        modal::{modal_panel, ModalStyle},
    },
};

use super::{AuthenticationFlow, IntermediatePasswordSaver, UiAuthFlow};

#[derive(Component, TypePath)]
pub(super) struct AuthenticationModal;

#[derive(Component, TypePath)]
pub(super) struct SignInButton;

#[derive(Component, TypePath)]
pub(super) struct SignUpButton;

#[derive(Component, TypePath)]
pub(super) struct SwitchToSignUpButton;

#[derive(Component, TypePath)]
pub(super) struct SwitchToSignInButton;

#[derive(Component, TypePath)]
pub(super) struct EmailEntity;

#[derive(Component, TypePath)]
pub(super) struct PasswordEntity;

pub fn update_intermediate_password_resource(
    email: Query<Entity, With<EmailEntity>>,
    password: Query<Entity, With<PasswordEntity>>,
    mut text_query: Query<&mut Text, With<TextInputInner>>,
    children_query: Query<&Children>,
    mut password_resource: ResMut<IntermediatePasswordSaver>,
) {
    let Ok(email_entity) = email.get_single() else {
        return;
    };

    let Ok(password_entity) = password.get_single() else {
        return;
    };
    let mut email = String::default();
    for descendant in children_query.iter_descendants(email_entity) {
        if let Ok(text) = text_query.get_mut(descendant) {
            email = format!("{}{}", text.sections[0].value, text.sections[2].value);
        } else {
            continue;
        }
    }

    let mut password = String::default();

    for descendant in children_query.iter_descendants(password_entity) {
        if let Ok(text) = text_query.get_mut(descendant) {
            password = format!("{}{}", text.sections[0].value, text.sections[2].value);
        } else {
            continue;
        }
    }

    password_resource.email = email;
    password_resource.password = password;
}

pub(super) fn sign_in(
    state: Option<Res<UiAuthFlow>>,
    mut su: EventWriter<SignInEvent>,
    mut password_resource: ResMut<IntermediatePasswordSaver>,
    mut commands: Commands,
) {
    let last_state = match state {
        Some(state) => Some(state.authentication_flow.clone()),
        None => None,
    };
    su.send(SignInEvent {
        login_info: PasswordLoginInfo::new(
            &password_resource.email,
            &password_resource.password,
            true,
        ),
    });

    password_resource.password.clear();

    commands.insert_resource(UiAuthFlow {
        authentication_flow: AuthenticationFlow::AttemptingSignIn,
        last_state,
    });
}

pub(super) fn sign_up(
    state: Option<Res<UiAuthFlow>>,
    mut su: EventWriter<SignUpEvent>,
    mut password_resource: ResMut<IntermediatePasswordSaver>,
    mut commands: Commands,
) {
    let last_state = match state {
        Some(state) => Some(state.authentication_flow.clone()),
        None => None,
    };
    su.send(SignUpEvent {
        info: PasswordLoginInfo::new(&password_resource.email, &password_resource.password, true),
    });

    password_resource.password.clear();

    commands.insert_resource(UiAuthFlow {
        authentication_flow: AuthenticationFlow::VerifyEmail,
        last_state,
    });
}

pub(super) fn switch_to_sign_up(state: Option<Res<UiAuthFlow>>, mut commands: Commands) {
    let last_state = match state {
        Some(state) => Some(state.authentication_flow.clone()),
        None => None,
    };
    commands.insert_resource(UiAuthFlow {
        authentication_flow: AuthenticationFlow::EnterInformationSignUp,
        last_state,
    });
}

pub(super) fn switch_to_sign_in(state: Option<Res<UiAuthFlow>>, mut commands: Commands) {
    let last_state = match state {
        Some(state) => Some(state.authentication_flow.clone()),
        None => None,
    };
    commands.insert_resource(UiAuthFlow {
        authentication_flow: AuthenticationFlow::EnterInformationSignIn,
        last_state,
    });
}

pub(super) fn setup_sign_up(mut commands: Commands, colors: Res<CurrentColors>) {
    let modal = modal_panel(
        AuthenticationModal,
        ModalStyle {
            with_close_button: false,
            close_button_bundle: None::<()>,
            modal_size: Some((Val::Percent(100.0), Val::Percent(100.0))),
            outline: true,
        },
        &colors,
        &mut commands,
    );

    let sign_up_button = basic_button(
        SwitchToSignInButton,
        ButtonStyle {
            bundle: None::<()>,
            text: String::from("Sign In Instead"),
            font_size: 20.0,
        },
        &mut commands,
        &colors,
    );

    commands.entity(modal).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Arts Game",
                TextStyle {
                    font_size: 100.0,
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

    commands.entity(modal).push_children(&[sign_up_button]);

    let email_button = commands
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
            EmailEntity,
        ))
        .id();

    let password_button = commands
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
            PasswordEntity,
        ))
        .id();

    let sign_in_button = basic_button(
        SignUpButton,
        ButtonStyle {
            bundle: None::<()>,
            text: String::from("Sign Up"),
            font_size: 40.0,
        },
        &mut commands,
        &colors,
    );

    commands
        .entity(modal)
        .push_children(&[email_button, password_button, sign_in_button]);
}

pub(super) fn setup_sign_in(mut commands: Commands, colors: Res<CurrentColors>) {
    let modal = modal_panel(
        AuthenticationModal,
        ModalStyle {
            with_close_button: false,
            close_button_bundle: None::<()>,
            modal_size: Some((Val::Percent(100.0), Val::Percent(100.0))),
            outline: true,
        },
        &colors,
        &mut commands,
    );

    let sign_up_button = basic_button(
        SwitchToSignUpButton,
        ButtonStyle {
            bundle: None::<()>,
            text: String::from("Sign Up Instead"),
            font_size: 20.0,
        },
        &mut commands,
        &colors,
    );

    commands.entity(modal).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "Arts Game",
                TextStyle {
                    font_size: 100.0,
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

    commands.entity(modal).push_children(&[sign_up_button]);

    let email_button = commands
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
            EmailEntity,
        ))
        .id();

    let password_button = commands
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
            PasswordEntity,
        ))
        .id();

    let sign_in_button = basic_button(
        SignInButton,
        ButtonStyle {
            bundle: None::<()>,
            text: String::from("Sign In"),
            font_size: 40.0,
        },
        &mut commands,
        &colors,
    );

    commands
        .entity(modal)
        .push_children(&[email_button, password_button, sign_in_button]);
}

pub fn handle_failed_sign_up(
    state: Option<Res<UiAuthFlow>>,
    mut su: EventReader<SignUpResultEvent>,
    mut commands: Commands,
) {
    for event in su.read() {
        if event.result.is_err() {
            let last_state = state
                .as_ref()
                .map(|state| state.authentication_flow.clone());

            commands.insert_resource(UiAuthFlow {
                authentication_flow: AuthenticationFlow::EnterInformationSignIn,
                last_state,
            });
        }
    }
}
