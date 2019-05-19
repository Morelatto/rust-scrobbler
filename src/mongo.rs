// use mongodb::{Bson, bson, doc};
// use mongodb::{Client, ThreadedClient};
// use mongodb::db::ThreadedDatabase;
// use mongodb::cursor::Cursor;

// // fn main() {
// //     let client = Client::connect("localhost", 27017).expect("Failed to initialize standalone client.");
// //     let coll = client.db("test").collection("movies");

// //     let doc = doc! {
// //         "title": "Jaws",
// //         "artist": "",
// //         "album": "",
// //         "scrobbled": "" // date
// //     };

// //     // Insert document into 'test.movies' collection
// //     coll.insert_one(doc.clone(), None).ok().expect("Failed to insert document.");

// //     // Find the document and receive a cursor
// //     let mut cursor = coll.find(Some(doc.clone()), None).ok().expect("Failed to execute find.");

// //     // let item = cursor.next();

// //     for item in cursor {
// //         // cursor.next() returns an Option<Result<Document>>
// //         match item {
// //             Some(Ok(doc)) => 
// //                 scrobble(doc.get("title"), doc.get("artist"), doc.get("album"), doc.get("scrobbled")),
// //             //     match doc.get("title") {
// //             //         Some(&Bson::String(ref title)) => println!("{}", title),
// //             //         _ => panic!("Expected title to be a string!"),
// //             // },
// //             Some(Err(_)) => 
// //                 panic!("Failed to get next from server!"),
// //             None => 
// //                 panic!("Server returned no results!"),
// //         }
// //     }
// // }

// pub fn get_not_scrobbled() -> Result<Cursor, String> {
//     let client = Client::connect("localhost", 27017).expect("Failed to start Mongo");
//     let coll = client.db("test").collection("movies");

//     let doc = doc!{"scrobbled": { "$exists": false }};

//     let mut cursor = coll.find(Some(doc.clone()), None).ok().expect("Failed to execute find.");
//     Ok(cursor)

//     // coll.update_one()



// }

// // pub fn mark_scrobble(id) {
// // coll.find_one_and_update
// // }

// // let mut cursor = coll.find(None, None).unwrap();
// // for result in cursor {
// //     let doc = result.expect("Received network error during cursor operations.");
// //     if let Some(&Bson::String(ref value)) = doc.get("spirit_animal") {
// //         println!("My spirit animal is {}", value);
// //     }
// // }

// // coll.update_one(doc!{}, doc!{ "director": "Robert Zemeckis" }, None).unwrap();


// //
// // ## Command Monitoring
// //
// // The driver provides an intuitive interface for monitoring and responding to runtime information
// // about commands being executed on the server. Arbitrary functions can be used as start and
// // completion hooks, reacting to command results from the server.
// //
// // ```no_run
// // # use mongodb::{Client, CommandResult, ThreadedClient};
// // fn log_query_duration(client: Client, command_result: &CommandResult) {
// //     match command_result {
// //         &CommandResult::Success { duration, .. } => {
// //             println!("Command took {} nanoseconds.", duration);
// //         },
// //         _ => println!("Failed to execute command."),
// //     }
// // }
// //
// // let mut client = Client::connect("localhost", 27017).unwrap();
// // client.add_completion_hook(log_query_duration).unwrap();
// // ```

// //
// // ## Topology Monitoring
// //
// // Each server within a MongoDB server set is monitored asynchronously for changes in status, and
// // the driver's view of the current topology is updated in response to this. This allows the
// // driver to be aware of the status of the server set it is communicating with, and to make server
// // selections appropriately with regards to the user-specified `ReadPreference` and `WriteConcern`.
// //
// // ## Connection Pooling
// //
// // Each server within a MongoDB server set is maintained by the driver with a separate connection
// // pool. By default, each pool has a maximum of 5 concurrent open connections.
