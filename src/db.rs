use mongodb::{
    bson::doc,
    sync::Client,
};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct Task {
    _id: String,
    description: String,
    completed: bool,
}

pub fn connect_db_sync() -> mongodb::error::Result<Client>{
    let uri = "mongodb+srv://ideans:code1@rusty1.znwsgiu.mongodb.net/";
    let client = Client::with_uri_str(uri)?;
    let database = client.database("TaskList");

    let collection_task = database.collection::<Task>("Task");

    // let docs = vec![
    //     Task {
    //         description: "The first task".to_string(),
    //         completed: false,
    //     },
    //     Task {
    //         description: "Another task added".to_string(),
    //         completed: true,
    //     }
    // ];

    // collection_task.insert_many(docs, None)?;

        let cursor = collection_task.find(None, None)?;
        for result in cursor {
            let task = result?;
            println!("id: {} Description: {}", task._id, task.description);
        }



    Ok(client)
}


// #[tokio::main]
// pub async fn connect_db() -> mongodb::error::Result<Client> {
//     println!("Buiilding connection to MongoDB Atlas server...");
//     let uri = "mongodb+srv://ideans:code1@rusty1.znwsgiu.mongodb.net/";
//     let mut client_options = ClientOptions::parse(uri);

//     // let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
//     // client_options.server_api = Some(server_api);

//     println!("    Attempting connection to MongoDB...");
//     let client = Client::with_options(client_options)?;
//     println!("    Connection established. Client created.\n");

//     println!("Databases found in cluster: ");
//     println!("----------------------------------------------");
//     for db_name in client.list_database_names(None, None).await? {
//         println!("{}", db_name);
//     }
//     println!("----------------------------------------------\n");

//     let db = client.database("TaskList");
//     println!("Collections found in database TaskList: ");
//     println!("----------------------------------------------");
//     for collection_name in db.list_collection_names(None).await? {
//         println!("{}", collection_name);
//     }
//     println!("----------------------------------------------\n");

//     // println!("Attempting ping command on database 'Rusty'");
//     // client
//     //     .database("rusty")
//     //     .run_command(doc! {"ping": 1}, None)
//     //     .await?;

//     Ok(client)
// }
