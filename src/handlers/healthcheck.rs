use actix_web::{web, HttpResponse, Scope};

use crate::{dtos::Response, error::HttpError};

pub fn healthcheck_scope() -> Scope {
    web::scope("/api/healthcheck").route("", web::get().to(healthcheck))
}

#[utoipa::path(
    get,
    path = "/api/healthcheck",
    tag = "Healthcheck Endpoint",
    responses(
        (status = 200, description= "Authenticated User", body = Response),       
    )
)]
pub async fn healthcheck() -> Result<HttpResponse, HttpError> {
    const MESSAGE: &str = "Complete Restful API in Rust";
    let response = Response {
        status: "success",
        message: MESSAGE.to_string(),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_healthcheck() {
        let app = test::init_service(
            App::new()
                .service(web::scope("/api/healthcheck").route("", web::get().to(healthcheck))),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/api/healthcheck")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        const MESSAGE: &str = "Complete Restful API in Rust";
        let response = Response {
            status: "success",
            message: MESSAGE.to_string(),
        };
        let expected_json = serde_json::json!(response);

        assert_eq!(body, serde_json::to_string(&expected_json).unwrap());
    }
}
