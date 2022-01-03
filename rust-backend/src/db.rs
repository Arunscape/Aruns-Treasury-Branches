use worker::*;

use postgres::{Client, NoTls, Error};

pub async fn get_balance(apikey: String) -> Result<Response>{

    let url = "postgresql://postgres:[YOUR-PASSWORD]@db.bkiuhzngtkoliiqqqunp.supabase.co:5432/postgres";
    let mut client = Client::connect()
    let endpoint = "https://bkiuhzngtkoliiqqqunp.supabase.co/rest/v1/";
    let client = Postgrest::new(endpoint)
        .insert_header("apikey", &apikey);


    let res = client.from("accounts")
        .auth(&apikey)
        .select("username,account_name,balance")
        .execute()
        .await
        .map_err(maperr)?;


    let res = res.text().await;

    let res = format!("{:?}", res);
    Response::ok(res)
}
