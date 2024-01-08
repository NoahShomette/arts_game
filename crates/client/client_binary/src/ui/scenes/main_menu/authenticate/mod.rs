use std::path::Path;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        change_detection::DetectChanges,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Res, Resource, SystemId},
    },
    reflect::TypePath,
};

use bevy_persistent::{Persistent, StorageFormat};
use core_library::authentication::{
    client_authentication::EmailPasswordCredentials, AppAuthenticationState,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    ui::{
        marker_component, scenes::cleanup_scene, widgets::basic_button::BasicButtonAppExtension,
        UiSystemIdResource,
    },
};

use self::{
    attempting_sign_in::{handle_failed_login, setup_attempt_sign_in},
    sign_in::{
        handle_failed_sign_up, setup_sign_in, setup_sign_up, sign_in, sign_up, switch_to_sign_in,
        switch_to_sign_up, update_intermediate_password_resource, AuthenticationModal,
        SignInButton, SignUpButton, SwitchToSignInButton, SwitchToSignUpButton,
    },
    verify_email::{setup_verify_email, VerifyEmailPlugin},
};

mod attempting_sign_in;
mod sign_in;
mod verify_email;

pub(super) struct AuthenticatePlugin;

impl Plugin for AuthenticatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(VerifyEmailPlugin);
        app.init_resource::<IntermediatePasswordSaver>();
        let enter_information_sign_in = app.world.register_system(setup_sign_in);
        let enter_information_sign_up = app.world.register_system(setup_sign_up);
        let verify_email = app.world.register_system(setup_verify_email);
        let attempting_sign_in = app.world.register_system(setup_attempt_sign_in);

        let cleanup_auth_modal = app
            .world
            .register_system(cleanup_scene::<AuthenticationModal>);

        app.insert_resource(AuthFlowSetupSystems {
            enter_information_sign_in: (enter_information_sign_in, cleanup_auth_modal),
            enter_information_sign_up: (enter_information_sign_up, cleanup_auth_modal),
            verify_email: (verify_email, cleanup_auth_modal),
            attempting_sign_in: (attempting_sign_in, cleanup_auth_modal),
        });

        app.init_resource::<FirstSignIn>();

        app.add_systems(Startup, load_login_info);

        app.add_systems(
            OnEnter(AppState::MainMenu),
            (setup_auth_ui, sign_in_if_saved_info)
                .run_if(in_state(AppAuthenticationState::NotAuthenticated)),
        );

        app.add_systems(
            OnEnter(AppAuthenticationState::Authenticated),
            cleanup_scene::<AuthenticationModal>,
        );

        app.add_systems(
            Update,
            (
                handle_change_auth_states,
                update_intermediate_password_resource,
                handle_failed_login,
                handle_failed_sign_up,
            )
                .run_if(in_state(AppAuthenticationState::NotAuthenticated)),
        );

        app.add_button_handler(marker_component::<SignInButton>(), sign_in);
        app.add_button_handler(marker_component::<SignUpButton>(), sign_up);
        app.add_button_handler(
            marker_component::<SwitchToSignInButton>(),
            switch_to_sign_in,
        );
        app.add_button_handler(
            marker_component::<SwitchToSignUpButton>(),
            switch_to_sign_up,
        );
    }
}

#[derive(Resource)]
pub struct UiAuthFlow {
    authentication_flow: AuthenticationFlow,
    last_state: Option<AuthenticationFlow>,
}
#[derive(Resource)]
pub struct AuthFlowSetupSystems {
    enter_information_sign_up: (SystemId, SystemId),
    enter_information_sign_in: (SystemId, SystemId),
    verify_email: (SystemId, SystemId),
    attempting_sign_in: (SystemId, SystemId),
}

#[derive(Clone, PartialEq, Eq)]
enum AuthenticationFlow {
    EnterInformationSignIn,
    EnterInformationSignUp,
    VerifyEmail,
    AttemptingSignIn,
}

#[derive(Resource, Default)]
struct IntermediatePasswordSaver {
    pub email: String,
    pub password: String,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct SavedEmailPasswordCredentials(pub Option<EmailPasswordCredentials>);

#[derive(Resource, Default)]
pub struct FirstSignIn;

fn handle_change_auth_states(
    auth_flow: Option<Res<UiAuthFlow>>,
    mut commands: Commands,
    auth_flow_systems: Res<AuthFlowSetupSystems>,
) {
    if let Some(auth_flow) = auth_flow {
        if !auth_flow.is_changed() {
            return;
        }

        if let Some(last_state) = &auth_flow.last_state {
            match last_state {
                AuthenticationFlow::EnterInformationSignIn => {
                    commands.run_system(auth_flow_systems.enter_information_sign_in.1)
                }
                AuthenticationFlow::EnterInformationSignUp => {
                    commands.run_system(auth_flow_systems.enter_information_sign_up.1)
                }
                AuthenticationFlow::VerifyEmail => {
                    commands.run_system(auth_flow_systems.verify_email.1)
                }
                AuthenticationFlow::AttemptingSignIn => {
                    commands.run_system(auth_flow_systems.attempting_sign_in.1)
                }
            }
        }

        match auth_flow.authentication_flow {
            AuthenticationFlow::EnterInformationSignIn => {
                commands.run_system(auth_flow_systems.enter_information_sign_in.0);
            }
            AuthenticationFlow::EnterInformationSignUp => {
                commands.run_system(auth_flow_systems.enter_information_sign_up.0)
            }
            AuthenticationFlow::AttemptingSignIn => {
                commands.run_system(auth_flow_systems.attempting_sign_in.0)
            }
            AuthenticationFlow::VerifyEmail => {
                commands.run_system(auth_flow_systems.verify_email.0)
            }
        }
    }
}

fn setup_auth_ui(mut commands: Commands) {
    commands.insert_resource(UiAuthFlow {
        authentication_flow: AuthenticationFlow::EnterInformationSignIn,
        last_state: None,
    });
}

fn load_login_info(mut commands: Commands) {
    let config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("arts-game"))
        .unwrap_or(Path::new("local").join("configuration"));

    commands.insert_resource(
        Persistent::<SavedEmailPasswordCredentials>::builder()
            .name("saved sign in info")
            .format(StorageFormat::Json)
            .path(config_dir.join("saved-sign-in-info.json"))
            .default(SavedEmailPasswordCredentials(None))
            .build()
            .expect("failed to initialize key bindings"),
    )
}

fn sign_in_if_saved_info(
    resource: Res<UiSystemIdResource>,
    mut commands: Commands,
    option_first_sign: Option<Res<FirstSignIn>>,
) {
    if option_first_sign.is_none() {
        return;
    }

    commands.run_system(
        *resource
            .map
            .get(SignInButton::type_path())
            .expect("Failed to register sign in button. Expected in sign_in_if_saved_info"),
    );
}
