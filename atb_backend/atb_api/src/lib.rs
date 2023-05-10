use async_trait::async_trait;

wai_bindgen_rust::export!("apiclient.wai");

const BASEURL: &'static str = "http://localhost:8080";
pub struct Apiclient;


#[async_trait(?Send)]
impl apiclient::Apiclient for Apiclient{
    async fn helloworld() -> String {
        let url = format!("{}/", BASEURL);
        let resp = reqwest::get(&url).await.unwrap();
        resp.text().await.unwrap()
    }
   
}