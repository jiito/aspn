use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use reqwest::multipart;

pub async fn upload_pando() -> std::io::Result<()> {
    let form = reqwest::multipart::Form::new();
    let file = File::open("/Users/bjar/foo.txt")?;

    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    let file = multipart::Part::bytes(buffer).file_name("/Users/bjar/foo.txt");
    let form = form.part("file", file);
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8080/upload")
        .multipart(form)
        .send()
        .await;
    println!("Sent file away!");

    Ok(())
}
