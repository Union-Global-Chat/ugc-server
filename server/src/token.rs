use crate::error::APIError;
use crate::state::AppState;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use db::token::get_token;

pub struct Token {
    pub bot_id: i64,
}

#[async_trait]
impl FromRequestParts<AppState> for Token {
    type Rejection = APIError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| APIError::unauthorized("Missing authorization header"))?;

        if let Some(bot_id) = get_token(&state.pool, bearer.token().to_string()).await? {
            return Ok(Token { bot_id });
        }

        Err(APIError::unauthorized("Invalid token"))
    }
}
