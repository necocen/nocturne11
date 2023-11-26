use anyhow::Context;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};

use crate::{
    adapters::{AppConfigProvider, GoogleCertsProvider},
    ApplicationResult,
};

pub struct AuthenticateUseCase;

impl AuthenticateUseCase {
    pub async fn execute(
        app_config: &impl AppConfigProvider,
        certs: &impl GoogleCertsProvider,
        jwt: &str,
    ) -> ApplicationResult<String> {
        // FIXME: たぶんこれはAuthenticationClientに委譲すべきで、audienceとかsubjectが正しいかどうかだけをUseCaseで確認すべきな気がする
        const ISSUERS: [&str; 2] = ["accounts.google.com", "https://accounts.google.com"];
        let header = decode_header(jwt)?;
        let kid = header
            .kid
            .context("JWT token does not contain 'kid' field.")?;
        let Some((n, e)) = certs.get_by_key(&kid).await? else {
            Err(anyhow::anyhow!("cert not found"))?
        };
        let key = DecodingKey::from_rsa_components(&n, &e)?;
        let mut validation = Validation::new(Algorithm::RS256);
        let auth_settings = app_config
            .get_auth_settings()
            .context("failed to get app config")?;
        validation.sub = Some(auth_settings.admin_user_id);
        validation.set_audience([auth_settings.google_client_id].as_ref());
        validation.set_issuer(&ISSUERS);

        #[derive(serde::Deserialize)]
        struct Claims {
            sub: String,
        }
        let data = decode::<Claims>(jwt, &key, &validation)?;

        // IDを返す
        Ok(data.claims.sub)
    }
}
