use rocket::{post, State};
use serde::Serialize;

use oauth_fcm::{create_shared_token_manager, send_fcm_message, SharedTokenManager};

#[derive(Serialize)]
struct MyData {
    message: String,
    count: i32,
}

#[post("/send")]
async fn send_notification(token_manager: &State<SharedTokenManager>) -> Result<String, String> {
    // It is a good idea to load these from an .env file. Additionally, you can store them in a shared `Config` state.
    let device_token = "YOUR_DEVICE_TOKEN";
    let project_id = "YOUR_PROJECT_ID";
    let data = MyData {
        message: "Hello from Rocket!".to_string(),
        count: 42,
    };

    send_fcm_message(
        device_token,
        None,
        Some(data),
        token_manager.inner(),
        project_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok("FCM message sent successfully".to_string())
}

#[rocket::main]
async fn main() {
    let shared_token_manager =
        create_shared_token_manager("path/to/google/credentials.json").unwrap();

    rocket::build()
        .manage(shared_token_manager)
        .mount("/", rocket::routes![send_notification])
        .launch()
        .await
        .unwrap();
}
