use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

// #[tokio::connect_db]
pub async fn connect_db() -> mongodb::error::Result<()> {
    println!("Buiilding connection to MongoDB Atlas server...");
    let uri = "mongodb+srv://ideans:code1@rusty1.znwsgiu.mongodb.net/";
    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    println!("    Attempting connection to MongoDB...");
    let client = Client::with_options(client_options)?;
    println!("    Connection established. Client created.");

    println!("Attempting ping command on database 'Rusty'");
    client
        .database("rusty")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    Ok(())
}