use self::templates::UnauthorizedTemplate;
use crate::{askama_helpers::TemplateToResponse, context::AppContext};
use actix_web::{dev::ServiceResponse, middleware::ErrorHandlerResponse, Result};

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

mod templates {
    use crate::context::AppContext;
    use askama::Template;

    #[derive(Template)]
    #[template(path = "errors/unauthorized.html")]
    pub struct UnauthorizedTemplate {
        pub context: AppContext,
    }
}
