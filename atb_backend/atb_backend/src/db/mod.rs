use atb_types::prelude::*;
use lazy_static::lazy_static;
use sqlx::{query, query_as};
use std::env;
use uuid::Uuid;

lazy_static! {
    static ref DB_URL: String =
        env::var("DATABASE_URL").unwrap_or("postgres://postgres@localhost/postgres".into());
}

pub struct ATBError(String);

use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct ATBDB {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ATBDB {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*DB_URL)
            .await?;
        Ok(Self { pool })
    }

    /// Takes in a minecraft UUID
    pub async fn add_user(&mut self, id: Uuid) -> Result<User, sqlx::Error> {
        let user = query_as!(User, "INSERT INTO users (id) VALUES ($1) ON CONFLICT (id) DO UPDATE SET id = users.id RETURNING *", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn new_account(&mut self, userid: Uuid, nickname: String) -> Result<Account, sqlx::Error> {

        let account = query_as!(Account,
            "INSERT INTO accounts (userid, nickname) VALUES ($1, $2) RETURNING *",
            userid,
            nickname,
        )
        .fetch_one(&self.pool)
        .await?;
        

        Ok(account)
    }

    pub async fn change_account_name(
        &mut self,
        userid: Uuid,
        old_nickname: &str,
        new_nickname: &str,   
    ) -> Result<Account, sqlx::Error> {
        let account = query_as!(Account,
            "UPDATE accounts SET nickname = $1 WHERE nickname = $2 AND userid = $3 RETURNING *",
            new_nickname,
            old_nickname,
            userid,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(account)
    }

    pub async fn delete_account(&mut self, accountid: Uuid, userid: Uuid) -> Result<(), sqlx::Error> {
        let _account = query_as!(Account, "DELETE FROM accounts WHERE id = $1 and userid = $2 RETURNING *", accountid, userid)
            .fetch_one(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_accounts_for_user(&mut self, id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
        let accounts = query_as!(Account, "SELECT * FROM accounts WHERE userid = $1", id)
            .fetch_all(&self.pool)
            .await?;

        Ok(accounts)
    }

    pub async fn get_transactions_for_account(
        &mut self,
        id: Uuid,
    ) -> Result<Vec<Transaction>, sqlx::Error> {
        let transactions = query_as!(
            Transaction,
            "SELECT * FROM transactions WHERE fromid = $1 OR toid = $1",
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    pub async fn get_balances_for_account(
        &mut self,
        id: Uuid,
    ) -> Result<Vec<Balance>, sqlx::Error> {
        let balances = query_as!(Balance, "SELECT * FROM balances WHERE accountid = $1", id)
            .fetch_all(&self.pool)
            .await?;

        Ok(balances)
    }

    pub async fn process_transaction(
        &mut self,
        fromid: Uuid,
        toid: Uuid,
        quantity: i64,
        item: &str,
        price: i64,
    ) -> Result<Transaction, sqlx::Error> {
        // BEGIN;
        // UPDATE balances SET amount = (amount - $3) WHERE accountid = $1 AND item = $4;
        // UPDATE balances SET amount = (amount + $3) WHERE accountid = $2 AND item = $4;

        // INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *;
        // COMMIT;
        // ",
        // fromid,
        // toid,
        // quantity,
        // item,
        // price

        let mut transaction = self.pool.begin().await?;
        query!(
            "UPDATE balances SET quantity = (quantity - $1) WHERE accountid = $2 AND item = $3",
            quantity,
            fromid,
            item
        )
        .execute(&mut transaction)
        .await?;

        query!(
            "UPDATE balances SET quantity = (quantity + $1) WHERE accountid = $2 AND item = $3",
            quantity,
            toid,
            item
        )
        .execute(&mut transaction)
        .await?;

        let t = query_as!(Transaction, "INSERT INTO transactions (fromid, toid, item, quantity, price) VALUES ($1, $2, $3, $4, $5) RETURNING *", fromid, toid, item, quantity, price)
            .fetch_one(&mut transaction)
            .await?;

        transaction.commit().await?;

        Ok(t)
    }

    pub async fn deposit(
        &mut self,
        accountid: Uuid,
        item: &str,
        quantity: i64,
    ) -> Result<Balance, sqlx::Error> {
        let balance = query_as!(
            Balance,
            "
        INSERT INTO balances (accountid, item, quantity) VALUES ($1, $2, $3)
        ON CONFLICT (accountid, item) DO UPDATE SET quantity = (balances.quantity + $3)
        RETURNING *;
      
        ",
            accountid,
            item,
            quantity
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(balance)
    }

    pub async fn withdraw(
        &mut self,
        accountid: Uuid,
        item: &str,
        quantity: i64,
    ) -> Result<Balance, sqlx::Error> {
        let balance = query_as!(
            Balance,
            "
        UPDATE balances SET quantity = (quantity - $3) WHERE accountid = $1 AND item = $2
        RETURNING *;
      
        ",
            accountid,
            item,
            quantity
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(balance)
    }
}
