use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    Connection, ConnectionProperties, BasicProperties, ExchangeKind,
};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await?;

    let channel = conn.create_channel().await?;
    
    // Declare exchange
    channel
        .exchange_declare(
            "user_created",
            ExchangeKind::Topic,
            lapin::options::ExchangeDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    let messages = vec![
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406404642-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406404642-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406404642-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406404642-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406404642-Emir".to_owned(),
        },
    ];

    for msg in messages {
        let payload = borsh::to_vec(&msg)?;
        channel
            .basic_publish(
                "user_created",
                "user.created",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?;
        println!("Published: {:?}", msg);
    }

    Ok(())
}
