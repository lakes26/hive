use super::account_response::AccountResponse;
use leptos::*;

#[server]
pub async fn edit_account(
    new_email: String,
    new_password: String,
    new_password_confirmation: String,
    password: String,
) -> Result<AccountResponse, ServerFnError> {
    use crate::functions::auth::identity::uuid;
    use crate::functions::db::pool;
    use argon2::{
        password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
        Argon2,
    };
    use db_lib::models::user::User;
    use rand_core::OsRng;

    if new_password != new_password_confirmation {
        return Err(ServerFnError::ServerError(
            "Passwords don't match.".to_string(),
        ));
    }

    let pool = pool()?;

    let uuid = uuid()?;
    let user = User::find_by_uuid(&uuid, &pool).await?;

    let argon2 = Argon2::default();
    let parsed_hash =
        PasswordHash::new(&user.password).map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    if argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(new_password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?
        .to_string();

    user.edit(&hashed_password, &new_email, &pool).await?;
    leptos_actix::redirect("/");
    AccountResponse::from_uuid(&user.id, &pool).await
}
