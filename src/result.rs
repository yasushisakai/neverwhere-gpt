use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GptResult {
    choices: Vec<ChoiceResult>,
}

impl GptResult {
    pub fn rate_city_result(&self) -> Result<RateResult, serde_json::Error> {
        let args = self.choices[0].message.function_call.arguments.to_string();
        serde_json::from_str::<CityRateResult>(&args).map(|r| r.into())
    }

    pub fn rate_feature_result(&self) -> Result<RateResult, serde_json::Error> {
        let args = self.choices[0].message.function_call.arguments.to_string();
        serde_json::from_str::<FeatureRateResult>(&args).map(|r| r.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ChoiceResult {
    message: MessageResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageResult {
    function_call: FunctionCallResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionCallResult {
    name: String,
    arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateResult {
    target: String,
    score: u32,
}

impl From<CityRateResult> for RateResult {
    fn from(value: CityRateResult) -> Self {
        Self {
            target: value.city_name,
            score: value.score,
        }
    }
}

impl From<FeatureRateResult> for RateResult {
    fn from(value: FeatureRateResult) -> Self {
        Self {
            target: value.feature_name,
            score: value.score,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityRateResult {
    city_name: String,
    score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureRateResult {
    feature_name: String,
    score: u32,
}
