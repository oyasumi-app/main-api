use axum::Json;

use crate::{ExtractUser, User};

use api_types::v1::token_info::*;

pub async fn check(ExtractUser(user): ExtractUser) -> Json<CheckResponse> {
    match user {
        User::Anonymous => Json(CheckResponse::Anonymous),
        User::InvalidToken => Json(CheckResponse::InvalidToken),
        User::ValidToken((user, token)) => Json(CheckResponse::ValidToken(TokenData {
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
