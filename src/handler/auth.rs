use crate::errors;
use crate::errors::ServiceError;
use actix_web::error::BlockingError;
use actix_web::{http, web, HttpResponse, Result};
use futures::Future;
use oidc;
use reqwest;

// TODO: dotenv で取ってきたい
const CLIENT_ID: &str = "xxx";
const CLIENT_SECRET: &str = "xxx";

pub fn auth() -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || {
        // TODO: Hostの名前は動的にとってきたい
        let redirect_url = reqwest::Url::parse("https://auth.suzurin.me/callback")
            .map_err(|_| ServiceError::InternalServerError)?;
        // TODO: dotenvでとってきたい
        let issuer = reqwest::Url::parse("xxx").map_err(|_| ServiceError::InternalServerError)?;

        oidc::Client::discover(CLIENT_ID.into(), CLIENT_SECRET.into(), redirect_url, issuer)
            .map_err(|err| {
                println!("{}", err);
                ServiceError::InternalServerError
            })
    })
    .then(|res| match res {
        Ok(client) => {
            let auth_url = client.auth_url(&Default::default());
            Ok(HttpResponse::TemporaryRedirect()
                .header(http::header::LOCATION, auth_url.as_str())
                .finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    })
}

pub fn callback(param: web::Query<CallbackParams>) -> HttpResponse {
    // TODO: ログインをcookieに書き込みたい
    HttpResponse::Ok().json(format!("{}", param.code))
}

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
}
