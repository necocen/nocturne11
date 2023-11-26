use self::templates::{
    BadRequestTemplate, InternalErrorTemplate, NotFoundTemplate, UnauthorizedTemplate,
};
use crate::{context::AppContext, errors::Error};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::ServiceResponse,
    http::{header, StatusCode},
    middleware::ErrorHandlerResponse,
    HttpMessage, HttpResponse, HttpResponseBuilder, ResponseError, Result,
};
use askama_actix::TemplateToResponse;

pub fn error_400(res: ServiceResponse<BoxBody>) -> Result<ErrorHandlerResponse<BoxBody>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        let (req, res) = res.into_parts();
        let status = res.status();
        // FIXME: resがStreamだと.try_into_bytes().unwrap()で落ちる
        let result = res
            .into_body()
            .try_into_bytes()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let body = std::str::from_utf8(&result)?.to_owned();
        let mut res = BadRequestTemplate { context, body }.to_response();
        *res.status_mut() = status;
        Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
            req,
            res.map_into_left_body(),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    }
}

pub fn error_401(res: ServiceResponse<BoxBody>) -> Result<ErrorHandlerResponse<BoxBody>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        let body = UnauthorizedTemplate { context }.to_response().into_body();
        Ok(ErrorHandlerResponse::Response(
            res.map_body(|_, _| body).map_into_left_body(),
        ))
    } else {
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    }
}

pub fn error_404(res: ServiceResponse<BoxBody>) -> Result<ErrorHandlerResponse<BoxBody>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        let (req, res) = res.into_parts();
        let status = res.status();
        // FIXME: resがStreamだと.try_into_bytes().unwrap()で落ちる
        let result = res
            .into_body()
            .try_into_bytes()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let mut body = std::str::from_utf8(&result)?.to_owned();
        if body.is_empty() {
            body = "指定されたファイルが見つかりませんでした。".to_owned();
        }
        let mut res = NotFoundTemplate { context, body }.to_response();
        *res.status_mut() = status;
        Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
            req,
            res.map_into_left_body(),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    }
}

pub fn error_500(res: ServiceResponse<BoxBody>) -> Result<ErrorHandlerResponse<BoxBody>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        let (req, res) = res.into_parts();
        let status = res.status();
        // FIXME: resがStreamだと.try_into_bytes().unwrap()で落ちる
        let result = res
            .into_body()
            .try_into_bytes()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let body = std::str::from_utf8(&result)?.to_owned();
        let mut res = InternalErrorTemplate { context, body }.to_response();
        *res.status_mut() = status;
        Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
            req,
            res.map_into_left_body(),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use application::errors::ApplicationError::PostNotFound;
        use domain::Error::{Jwt, JwtIssuer};
        match self {
            Self::NoResult(_) => StatusCode::NOT_FOUND,
            Self::Domain(Jwt(_)) => StatusCode::BAD_REQUEST,
            Self::Domain(JwtIssuer(_)) => StatusCode::BAD_REQUEST,
            Self::Application(PostNotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        use application::errors::ApplicationError::PostNotFound;
        use domain::Error::{Jwt, JwtIssuer};
        match self {
            Self::Application(PostNotFound) => HttpResponseBuilder::new(self.status_code())
                .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                .body("指定されたIDの記事が見つかりませんでした。"),
            Self::NoResult(message) => HttpResponseBuilder::new(self.status_code())
                .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                .body(message.clone()),
            Self::Domain(Jwt(error)) => HttpResponseBuilder::new(self.status_code())
                .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                .body(format!("認証エラー: {}", error)),
            Self::Domain(JwtIssuer(issuer)) => HttpResponseBuilder::new(self.status_code())
                .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                .body(format!("予期しないJWT issuer: {}", issuer)),
            _ => {
                use std::error::Error;
                let msg = if let Some(source) = self.source() {
                    format!("{} (from {})", self, source)
                } else {
                    self.to_string()
                };
                HttpResponseBuilder::new(self.status_code())
                    .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                    .body(msg)
            }
        }
    }
}

mod templates {
    use crate::context::AppContext;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "errors/bad_request.html")]
    pub struct BadRequestTemplate {
        pub context: AppContext,
        pub body: String,
    }

    #[derive(Template)]
    #[template(path = "errors/unauthorized.html")]
    pub struct UnauthorizedTemplate {
        pub context: AppContext,
    }

    #[derive(Template)]
    #[template(path = "errors/not_found.html")]
    pub struct NotFoundTemplate {
        pub context: AppContext,
        pub body: String,
    }

    #[derive(Template)]
    #[template(path = "errors/internal_error.html")]
    pub struct InternalErrorTemplate {
        pub context: AppContext,
        pub body: String,
    }
}
