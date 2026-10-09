use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::server::{
    auth::{AxumSession, check_permission},
    permission::Permission,
    users::DynUsers,
};
#[cfg(feature = "server")]
use dioxus_server::{axum::Extension, http};

#[post("/user/signin", auth: AxumSession, Extension(users): Extension<DynUsers>)]
pub async fn sign_in(username: String, password: String) -> Result<()> {
    auth.login_user(users.sign_in(username, password).await?.id());
    Ok(())
}

#[post("/user/signout", auth: AxumSession)]
pub async fn sign_out() -> Result<()> {
    auth.logout_user();
    Ok(())
}

#[post("/user/signup", Extension(users): Extension<DynUsers>)]
pub async fn sign_up(username: String, password: String) -> Result<()> {
    users.sign_up(username, password).await?;
    Ok(())
}

#[get("/user/name", auth: AxumSession, Extension(users): Extension<DynUsers>)]
pub async fn user_name() -> Result<String> {
    check_permission(&users, http::Method::GET, Permission::default(), &auth).await?;
    Ok(users.find(auth.id).await?.name().to_owned())
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use std::sync::Arc;

    use axum_test::TestServer;
    use dioxus::server::{
        DioxusRouterExt, FullstackState,
        axum::{Extension, Router},
    };
    use tokio::sync::Mutex;

    use crate::user::server::users::{DynUsers, test::FakeUsers};

    #[derive(serde::Serialize)]
    struct SignUpBody {
        username: String,
        password: String,
    }

    fn test_server(fake: Arc<Mutex<FakeUsers>>) -> TestServer {
        let users: DynUsers = Arc::new(fake);
        let router = Router::<FullstackState>::new()
            .register_server_functions()
            .layer(Extension(users))
            .with_state(FullstackState::headless());
        TestServer::new(router)
    }

    #[tokio::test]
    async fn sign_up_registers_a_new_user() {
        let fake = Arc::new(Mutex::new(FakeUsers::default()));
        let server = test_server(fake.clone());

        let response = server
            .post("/user/signup")
            .json(&SignUpBody {
                username: "alice".to_owned(),
                password: "hunter2".to_owned(),
            })
            .await;

        response.assert_status_ok();

        let users = &fake.lock().await.users;
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name(), "alice");
    }
}
