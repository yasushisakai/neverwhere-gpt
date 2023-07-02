use crate::{
    request_body::RequestBody,
    result::{GptResult, RateResult},
};
use reqwest::{Client, Error};

const URL: &'static str = "https://api.openai.com/v1/chat/completions";

pub enum RateType {
    City(String),
    Feature(String),
}

pub struct RateRequest;

impl RateRequest {
    pub async fn rate(openai_key: &str, target: RateType) -> Result<RateResult, String> {
        let body = match &target {
            RateType::City(name) => RequestBody::rate_city(&name),
            RateType::Feature(aspect) => RequestBody::rate_feature(&aspect),
        };

        let token = format!("Bearer {}", openai_key);

        let client = Client::new();
        let response = client
            .post(URL)
            .header("Content-Type", "application/json")
            .header("Authorization", &token)
            .json(&body)
            .send()
            .await
            .map_err(|_e| "error sending request".to_string())?;

        let data: GptResult = response
            .json::<GptResult>()
            .await
            .map_err(|_e| "error result to json".to_string())?;

        let result = match target {
            RateType::City(_) => data.rate_city_result(),
            RateType::Feature(_) => data.rate_feature_result(),
        };

        result.map_err(|_e| "could not convert".to_string())
    }
}
