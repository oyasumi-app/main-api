use axum::Json;

use crate::{ExtractUser, LoginState};

use api_types::v1::token_info::*;

pub async fn check(ExtractUser(user): ExtractUser) -> Json<CheckResponse> {
    match user {
        LoginState::Anonymous => Json(CheckResponse::Anonymous),
        LoginState::InvalidToken => Json(CheckResponse::InvalidToken),
        LoginState::ValidToken((user, token)) => Json(CheckResponse::ValidToken(TokenData {
            user: TokenUserData {
                id: user.id,
                username: user.username,
                email: user.email,
            },
            token: TokenDetails {
                id: token.id,
                expires: token.expires,
            },
        })),
    }
}
