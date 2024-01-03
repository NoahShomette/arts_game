use bevy::{
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    prelude::default,
    text::{TextAlignment, TextStyle},
    ui::{node_bundles::TextBundle, AlignItems, JustifyContent, PositionType, Style, UiRect, Val},
};
use core_library::authentication::SignInResultEvent;

use crate::ui::{
    colors::CurrentColors,
    widgets::modal::{modal_panel, ModalStyle},
};

use super::{sign_in::AuthenticationModal, AuthenticationFlow, UiAuthFlow};

pub(super) fn setup_attempt_sign_in(mut commands: Commands, colors: Res<CurrentColors>) {
    let modal = modal_panel(
        AuthenticationModal,
        ModalStyle {
            can_close: false,
            close_button_bundle: None::<()>,
            modal_size: Some((Val::Percent(100.0), Val::Percent(100.0))),
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

        parent.spawn(
            TextBundle::from_section(
                "Attempting to sign in",
                TextStyle {
                    font_size: 50.0,
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
}

pub fn handle_failed_login(
    state: Option<Res<UiAuthFlow>>,
    mut su: EventReader<SignInResultEvent>,
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
