use postgrest::Postgrest;
use worker::*;


fn maperr(e: impl std::error::Error) -> worker::Error{
    worker::Error::from(e.to_string())
}

pub async fn get_balance(apikey: String) -> Result<Response>{
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
