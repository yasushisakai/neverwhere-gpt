use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    model: String,
    messages: Vec<Message>,
    functions: Vec<Function>,
    function_call: FunctionCall,
}

impl RequestBody {
    pub fn rate_city(name: &str) -> Self {
        let mut messages = Vec::new();

        messages.push(Message::rate_city_system());
        messages.push(Message::rate_city(name));

        Self {
            model: "gpt-3.5-turbo-0613".to_string(),
            messages,
            functions: vec![Function::rate_city()],
            function_call: FunctionCall::rate_city(),
        }
    }

    pub fn rate_feature(aspect: &str) -> Self {
        let mut messages = Vec::new();

        messages.push(Message::rate_feature_system());
        messages.push(Message::rate_feature(aspect));

        Self {
            model: "gpt-3.5-turbo-0613".to_string(),
            messages,
            functions: vec![Function::rate_feature()],
            function_call: FunctionCall::rate_feature(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

impl Message {
    fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }

    fn rate_city_system() -> Self {
        Self::new("system", "You are a urban planner that have lived all around the globe and experienced all cities. User will ask if a city is worth living. You should respond with a score and save the response using the rateCity function.")
    }

    fn rate_city(name: &str) -> Self {
        let content = format!("How did you like living in {}?", name);
        Self {
            role: "user".to_string(),
            content,
        }
    }

    fn rate_feature_system() -> Self {
        Self::new("system", "You are a urban planner that have lived all around the globe and experienced all cities. A User will ask one feature in a city. You should rate that feature how much it is important when living. You should respond with a score and save the response using the rateFeature function.")
    }

    fn rate_feature(aspect: &str) -> Self {
        let content = format!(
            "When living in a city, how much is {} important as a feature?",
            aspect
        );
        Self {
            role: "user".to_string(),
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Function {
    name: String,
    description: String,
    parameters: FunctionParameters,
}

impl Function {
    fn rate_city() -> Self {
        Self {
            name: "rateCity".to_string(),
            description: "saves if the assistant liked living in the city or not".to_string(),
            parameters: FunctionParameters::rate_city_params(),
        }
    }
    fn rate_feature() -> Self {
        Self {
            name: "rateFeature".to_string(),
            description: "saves how much a feature is important when living in a city".to_string(),
            parameters: FunctionParameters::rate_feature_params(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionParameters {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    properties: HashMap<String, FunctionProperty>,
    required: Vec<String>,
}

impl FunctionParameters {
    fn rate_city_params() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            "cityName".to_string(),
            FunctionProperty::new("string", "the city name"),
        );
        properties.insert("score".to_string(), FunctionProperty::new("number", "the score showing how much did planner enjoyed living in this city. The range should be 0 to 100"));
        Self {
            kind: "object".to_string(),
            properties,
            required: vec!["cityName".to_string(), "score".to_string()],
        }
    }

    fn rate_feature_params() -> Self {
        let mut properties = HashMap::new();
        properties.insert(
            "featureName".to_string(),
            FunctionProperty::new("string", "the city's feature"),
        );
        properties.insert("score".to_string(), FunctionProperty::new("number", "the score indicating how much the planner thought the feature is important when living in a given city. The range should be 0 to 100"));
        Self {
            kind: "object".to_string(),
            properties,
            required: vec!["featureName".to_string(), "score".to_string()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionProperty {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    kind: String,
    description: String,
}

impl FunctionProperty {
    fn new(kind: &str, description: &str) -> Self {
        Self {
            kind: kind.to_string(),
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionCall {
    name: String,
}

impl FunctionCall {
    fn rate_city() -> Self {
        Self {
            name: "rateCity".to_string(),
        }
    }
    fn rate_feature() -> Self {
        Self {
            name: "rateFeature".to_string(),
        }
    }
}
