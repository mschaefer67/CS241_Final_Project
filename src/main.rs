use firestore::*;
use serde::{Deserialize, Serialize};
use futures::stream::BoxStream;
use futures::StreamExt;

// pub fn config_env_var(name: &str) -> Result<String, String> {
//    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
// }

pub fn config_env_var() -> Result<String, String> {
    Ok("cs241-final-project".to_string())
 }
 

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
   some_id: String,
   some_string: String,
   one_more_string: String,
   some_num: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {//!
  // Create an instance
  let db = FirestoreDb::with_options_service_account_key_file(
    FirestoreDbOptions::new(config_env_var()?),
    "/Users/matt/CS241_Final_Project/cs241-final-project-4f09e1e04151.json".into()
    ).await?;

  const weather_readings: &'static str = "weather_readings";

  let my_struct = MyTestStructure {
       some_id: "test-1".to_string(),
       some_string: "Test".to_string(),
       one_more_string: "Test2".to_string(),
       some_num: 42,
  };

  // Create documents
  let object_returned: MyTestStructure = db.fluent()
      .insert()
      .into(weather_readings)
      .document_id(&my_struct.some_id)
      .object(&my_struct)
      .execute()
      .await?;

//   // Update documents
//   let object_updated: MyTestStructure = db.fluent()
//       .update()
//       .fields(paths!(MyTestStructure::{some_num, one_more_string})) // Update only specified fields
//       .in_col(weather_readings)
//       .document_id(&my_struct.some_id)
//       .object(&MyTestStructure {
//           some_num: my_struct.some_num + 1,
//          one_more_string: "updated-value".to_string(),
//           ..my_struct.clone()
//       })
//       .execute()
//      .await?;
 
//   // Get a document as an object by id
//   let find_it_again: Option<MyTestStructure> = db.fluent()
//         .select()
//         .by_id_in(weather_readings)
//         .obj()
//         .one(&my_struct.some_id)
//         .await?;

//   // Query and read stream of objects
//   let object_stream: BoxStream<MyTestStructure> = db.fluent()
//     .select()
//     .fields(paths!(MyTestStructure::{some_id, some_num, some_string, one_more_string})) // Optionally select the fields needed
//     .from(weather_readings)
//     .filter(|q| { // Fluent filter API example
//         q.for_all([
//             q.field(path!(MyTestStructure::some_num)).is_not_null(),
//             q.field(path!(MyTestStructure::some_string)).eq("Test"),
//             // Sometimes you have optional filters
//             Some("Test2")
//                 .and_then(|value| q.field(path!(MyTestStructure::one_more_string)).eq(value)),
//         ])
//     })
//     .order_by([(
//         path!(MyTestStructure::some_num),
//         FirestoreQueryDirection::Descending,
//     )])
//     .obj() // Reading documents as structures using Serde gRPC deserializer
//     .stream_query()
//     .await?;

//     let as_vec: Vec<MyTestStructure> = object_stream.collect().await;
//     println!("{:?}", as_vec);

//     // Delete documents
//     db.fluent()
//         .delete()
//         .from(weather_readings)
//         .document_id(&my_struct.some_id)
//         .execute()
//         .await?;

    Ok(())
}
