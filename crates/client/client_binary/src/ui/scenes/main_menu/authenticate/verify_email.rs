use bevy::{
    app::{Plugin, Update},
    ecs::{
        event::EventWriter,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Commands, Local, Res, ResMut},
    },
    hierarchy::BuildChildren,
    prelude::default,
    text::{TextAlignment, TextStyle},
    time::{Time, Timer},
    ui::{node_bundles::TextBundle, AlignItems, JustifyContent, PositionType, Style, UiRect, Val},
};
use core_library::{
    async_runners,
    authentication::{
        client_authentication::{IsUserEmailConfirmed, PasswordLoginInfo, SignInEvent},
        AppAuthenticationState, AuthenticationServerInfo, SignUpDetails,
    },
    network::HttpRequestMeta,
    AsyncChannel, TaskPoolRes,
};
use ehttp::Response;

use crate::ui::{
    colors::CurrentColors,
    widgets::modal::{modal_panel, ModalStyle},
};

pub struct VerifyEmailPlugin;

impl Plugin for VerifyEmailPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AsyncChannel::<VerifyEmailResponses>::default());
        app.add_systems(
            Update,
            (check_if_email_verified, handle_check_verify_responses)
                .run_if(in_state(AppAuthenticationState::NotAuthenticated)),
        );
    }
}

use super::{
    sign_in::AuthenticationModal, AuthenticationFlow, IntermediatePasswordSaver, UiAuthFlow,
};

pub(super) fn setup_verify_email(mut commands: Commands, colors: Res<CurrentColors>) {
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
                "Follow instructions in email sent to sign up email",
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

        parent.spawn(
            TextBundle::from_section(
                "App will automatically update when you complete verification",
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
        );
    });
}

#[derive(Clone)]
struct VerifyEmailResponses {
    response: Result<Response, String>,
}

struct EmailVerificationCheckTimer(Timer);

impl Default for EmailVerificationCheckTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, bevy::time::TimerMode::Repeating))
    }
}

#[allow(clippy::too_many_arguments)]
fn check_if_email_verified(
    auth_server_info: Res<AuthenticationServerInfo>,
    details: Option<Res<SignUpDetails>>,
    task_pool_res: Res<TaskPoolRes>,
    sign_up_channel: ResMut<AsyncChannel<VerifyEmailResponses>>,
    mut local_timer: Local<EmailVerificationCheckTimer>,
    password_resource: Res<IntermediatePasswordSaver>,
    state: Option<Res<UiAuthFlow>>,
    time: Res<Time>,
) {
    local_timer.0.tick(time.delta());

    if !local_timer.0.finished() {
        return;
    }

    if let Some(state) = state {
        if state.authentication_flow != AuthenticationFlow::VerifyEmail {
            return;
        }
    }

    let Some(_) = details else {
        return;
    };

    let addr = auth_server_info.addr.clone();
    let message = match serde_json::to_string(&HttpRequestMeta {
        request: IsUserEmailConfirmed {
            info: PasswordLoginInfo::new(
                &password_resource.email,
                &password_resource.password,
                true,
            ),
        },
    }) {
        Ok(message) => message.as_bytes().to_vec(),
        Err(_) => {
            return;
        }
    };

    let sign_up_channel = sign_up_channel.clone();
    let message = message.clone();

    if let Some(task) = async_runners::run_async(
        async move {
            let mut request =
                ehttp::Request::post(format!("{}auth/is_user_email_confirmed", addr), message);

            request
                .headers
                .insert("Content-Type".to_string(), "application/json".to_string());

            match ehttp::fetch_async(request).await {
                Ok(response) => {
                    let _ = sign_up_channel.sender_channel.send(VerifyEmailResponses {
                        response: Ok(response),
                    });
                }
                Err(err) => {
                    let _ = sign_up_channel
                        .sender_channel
                        .send(VerifyEmailResponses { response: Err(err) });
                }
            };
        },
        &task_pool_res.0,
    ) {
        task.detach();
    }
}

fn handle_check_verify_responses(
    sign_up_channel: ResMut<AsyncChannel<VerifyEmailResponses>>,
    mut su: EventWriter<SignInEvent>,
    state: Option<Res<UiAuthFlow>>,
    password_resource: Res<IntermediatePasswordSaver>,
    mut commands: Commands,
) {
    if let Ok(receiver) = sign_up_channel.reciever_channel.try_lock() {
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
                let is_verified = match serde_json::from_str::<bool>(result_text) {
                    Ok(result) => result,
                    Err(err) => {
                        println!("err: {} - on body {}", err, result_text);
                        return;
                    }
                };

                if is_verified {
                    let last_state = state
                        .as_ref()
                        .map(|state| state.authentication_flow.clone());

                    su.send(SignInEvent {
                        login_info: PasswordLoginInfo::new(
                            &password_resource.email,
                            &password_resource.password,
                            true,
                        ),
                    });

                    commands.insert_resource(UiAuthFlow {
                        authentication_flow: AuthenticationFlow::AttemptingSignIn,
                        last_state,
                    });
                }
            };
        }
    }
}
