use axum::{extract::Path, response::Json, routing::get, routing::post, Router};
use lambda_http::{run, Error};
use polars_lambda_axum::calculate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct BmiInput {
    height: f64,
    weight: f64,
}

#[derive(Serialize)]
struct BmiOutput {
    bmi: f64,
    category: String,
}

async fn root() -> &'static str {
    "Hello, Polars"
}

//simple url: /iris/filter/5
async fn get_filter(Path(value): Path<f64>) -> Json<Value> {
    let df = calculate(value).unwrap();
    let json = json!({
        "payload": format!("{}", df),
    });
    Json(json)
}

// New handler for BMI calculation
async fn calculate_bmi(Json(input): Json<BmiInput>) -> Json<BmiOutput> {
    let bmi = input.weight / (input.height * input.height);
    let category = match bmi {
        x if x < 18.5 => "Underweight",
        x if x < 25.0 => "Normal weight",
        x if x < 30.0 => "Overweight",
        _ => "Obese",
    };

    Json(BmiOutput {
        bmi: (bmi * 10.0).round() / 10.0, // Round to 1 decimal place
        category: category.to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let app = Router::new()
        .route("/hello", get(root))
        .route("/iris/filter/:value", get(get_filter))
        .route("/", post(calculate_bmi)); // New route for BMI calculation;
    run(app).await
}
