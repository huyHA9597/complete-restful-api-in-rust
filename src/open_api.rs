use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::dtos::{
    FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserData, UserListResponseDto,
    UserLoginResponseDto, UserResponseDto,
};
use crate::handlers::{auth, healthcheck, users};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,auth::logout,auth::register, 
        users::get_me, users::get_users, 
        healthcheck::healthcheck
    ),
    components(
        schemas(UserData,FilterUserDto,LoginUserDto,RegisterUserDto,UserResponseDto,UserLoginResponseDto,Response,UserListResponseDto)
    ),
    tags(
        (name = "Rust REST API", description = "Authentication in Rust Endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}
