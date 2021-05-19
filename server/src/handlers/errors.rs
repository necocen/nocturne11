use self::templates::{InternalErrorTemplate, NotFoundTemplate, UnauthorizedTemplate};
use crate::{askama_helpers::TemplateToResponse, context::AppContext, errors::Error};
use actix_web::{
    dev::{HttpResponseBuilder, ServiceResponse},
    http::{header, StatusCode},
    middleware::ErrorHandlerResponse,
    HttpResponse, ResponseError, Result,
};
use futures::{future::FutureExt, TryStreamExt};

pub fn error_401(res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        let body = UnauthorizedTemplate { context }.to_response()?.take_body();
        Ok(ErrorHandlerResponse::Response(res.map_body(|_, _| body)))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
    }
}

pub fn error_404(mut res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        Ok(ErrorHandlerResponse::Future(Box::pin(
            res.take_body().try_collect::<Vec<_>>().map(move |result| {
                let req = res.request().clone();
                let status = res.status();
                let mut body = std::str::from_utf8(&result?.concat())?.to_owned();
                if body.is_empty() {
                    body = "指定されたファイルが見つかりませんでした。".to_owned();
                }
                let mut res = NotFoundTemplate { context, body }.to_response()?;
                *res.status_mut() = status;
                Ok(ServiceResponse::new(req, res))
            }),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
    }
}

pub fn error_500(mut res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res
        .request()
        .clone()
        .extensions()
        .get::<AppContext>()
        .cloned()
    {
        Ok(ErrorHandlerResponse::Future(Box::pin(
            res.take_body().try_collect::<Vec<_>>().map(move |result| {
                let req = res.request().clone();
                let status = res.status();
                let body = std::str::from_utf8(&result?.concat())?.to_owned();
                let mut res = InternalErrorTemplate { context, body }.to_response()?;
                *res.status_mut() = status;
                Ok(ServiceResponse::new(req, res))
            }),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use domain::repositories::posts::Error::NotFound;
        use domain::Error::Posts;
        if let Self::Domain(Posts(NotFound(_))) = self {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        use domain::repositories::posts::Error::NotFound;
        use domain::Error::Posts;
        if let Self::Domain(Posts(NotFound(id))) = self {
            HttpResponseBuilder::new(self.status_code())
                .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
                .body(format!(
                    "指定されたID({})の記事が見つかりませんでした。",
                    id
                ))
        } else {
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

mod templates {
    use crate::context::AppContext;
    use askama::Template;

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
