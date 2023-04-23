use utoipa::{
    openapi::security::{Flow, OAuth2, Password, Scopes, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::get_token,
        crate::routes::create_user,
        crate::routes::get_logged_user,
        crate::routes::update_logged_user
    ),
    components(schemas(
        crate::errors::ProblemDetails,
        crate::models::auth::TokenPayload,
        crate::models::auth::LoginCredentials,
        crate::models::user::UserCreate,
        crate::models::user::User,
        crate::models::user::UserUpdate
    )),
    modifiers(&UserAuthAddon)
)]
struct ApiDocs;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger").url("/swagger/openapi.json", ApiDocs::openapi())
}

struct UserAuthAddon;

impl Modify for UserAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Bearer token",
                SecurityScheme::OAuth2(OAuth2::new([Flow::Password(Password::new(
                    "/api/auth/token",
                    Scopes::from_iter([("user", "Log in as user")]),
                ))])),
            )
        }
    }
}
