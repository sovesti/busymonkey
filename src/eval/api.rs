#[cfg(feature = "server")]
use crate::user::server::{auth::check_permission, permission::Permission, users::DynUsers};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus_server::{axum::Extension, http};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::{
    eval::server::{evaluator::evaluate_expression, expressions::DynExpressions},
    user::server::auth::AxumSession,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "server", derive(sqlx::FromRow))]
pub struct EvaluatedExpression {
    pub expression: String,
    pub result: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EvalResponse {
    pub result: String,
}

#[post("/eval", auth: AxumSession, Extension(users): Extension<DynUsers>, Extension(expressions): Extension<DynExpressions>)]
pub async fn eval_expr(expression: String) -> Result<EvalResponse> {
    check_permission(&users, http::Method::POST, Permission::default(), &auth)
        .await
        .or_forbidden("Permission denied")?;
    let result = evaluate_expression(&expression)
        .map_err(|error| HttpError::new(http::StatusCode::BAD_REQUEST, error.to_string()))?
        .to_string();
    record_expression(auth.id, expression, result.clone(), expressions).await?;
    Ok(EvalResponse { result })
}

#[cfg(feature = "server")]
async fn record_expression(
    id: i32,
    expression: String,
    result: String,
    expressions: DynExpressions,
) -> Result<()> {
    expressions
        .add_expression(id, EvaluatedExpression { expression, result })
        .await?;
    Ok(())
}

#[get("/eval", auth: AxumSession, expressions: Extension<DynExpressions>)]
pub async fn eval_history() -> Result<Vec<EvaluatedExpression>> {
    Ok(expressions.history(auth.id).await?)
}
