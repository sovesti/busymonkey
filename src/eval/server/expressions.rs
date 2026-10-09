use std::{fmt::Debug, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{PgPool, query_as};

use crate::eval::api::EvaluatedExpression;

#[async_trait]
pub trait Expressions: Debug + Send + Sync + 'static {
    async fn history(&self, account_id: i32) -> Result<Vec<EvaluatedExpression>>;
    async fn add_expression(
        &self,
        account_id: i32,
        expression: EvaluatedExpression,
    ) -> Result<EvaluatedExpression>;
}

pub type DynExpressions = Arc<dyn Expressions>;

#[async_trait]
impl Expressions for PgPool {
    async fn history(&self, account_id: i32) -> Result<Vec<EvaluatedExpression>> {
        Ok(query_as::<_, EvaluatedExpression>(
            r#"
            SELECT expression, result
            FROM evaluated_expression
            WHERE account_id = $1
            ORDER BY id DESC
            "#,
        )
        .bind(account_id)
        .fetch_all(self)
        .await?)
    }
    async fn add_expression(
        &self,
        account_id: i32,
        expression: EvaluatedExpression,
    ) -> Result<EvaluatedExpression> {
        Ok(query_as::<_, EvaluatedExpression>(
            r#"
            INSERT INTO evaluated_expression (account_id, expression, result)
            VALUES ($1, $2, $3)
            RETURNING expression, result
            "#,
        )
        .bind(account_id)
        .bind(expression.expression)
        .bind(expression.result)
        .fetch_one(self)
        .await?)
    }
}

#[cfg(test)]
pub mod tests {
    // TODO
}
