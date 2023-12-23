mod notification_service;

#[tokio::main]
async fn main() {
    // TODO: fill this
    let device_id = "";

    let res = notification_service::send_notification_to(&device_id).await;

    match res {
        Ok(_) => println!("notification sent"),
        Err(err) => println!("error: {}", err),
    }
}
