use anyhow::Context;
use domain::entities::config::Config;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};

use crate::{adapters::GoogleCertsProvider, ApplicationResult};

pub struct AuthenticateUseCase;

impl AuthenticateUseCase {
    pub async fn execute(
        app_config: &Config,
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
        validation.sub = Some(app_config.auth.admin_user_id.clone());
        validation.set_audience([app_config.auth.google_client_id.clone()].as_ref());
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
