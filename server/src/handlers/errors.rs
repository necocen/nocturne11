use self::templates::{InternalErrorTemplate, NotFoundTemplate, UnauthorizedTemplate};
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use actix_web::{dev::ServiceResponse, middleware::ErrorHandlerResponse, Result};
use futures::{future::FutureExt, TryStreamExt};

pub fn error_401(res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    let req = res.request();
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = req.clone().extensions().get::<AppContext>() {
        let context = context.clone();
        let body = UnauthorizedTemplate { context }.to_response()?.take_body();
        Ok(ErrorHandlerResponse::Response(res.map_body(|_, _| body)))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
    }
}

pub fn error_404(res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    let req = res.request();
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = req.clone().extensions().get::<AppContext>() {
        let context = context.clone();
        let body = NotFoundTemplate { context }.to_response()?.take_body();
        Ok(ErrorHandlerResponse::Response(res.map_body(|_, _| body)))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
    }
}

pub fn error_500(mut res: ServiceResponse) -> Result<ErrorHandlerResponse<actix_web::dev::Body>> {
    // AppContextが取得できればそれを使ってテンプレートを描画する
    // 取得できない場合は何もしない
    if let Some(context) = res.request().clone().extensions().get::<AppContext>() {
        let context = context.clone();
        Ok(ErrorHandlerResponse::Future(Box::pin(
            res.take_body().try_collect::<Vec<_>>().map(move |result| {
                let req = res.request().clone();
                let error = std::str::from_utf8(&result?.concat())?.to_owned();
                let res = InternalErrorTemplate { context, error }.to_response()?;
                Ok(ServiceResponse::new(req, res))
            }),
        )))
    } else {
        Ok(ErrorHandlerResponse::Response(res))
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
    }

    #[derive(Template)]
    #[template(path = "errors/internal_error.html")]
    pub struct InternalErrorTemplate {
        pub context: AppContext,
        pub error: String,
    }
}
