use google_cloud_auth::credentials;
use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_auth::Project;
use google_cloud_storage::client::Client;
use google_cloud_storage::client::ClientConfig;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::UploadObjectRequest;
use google_cloud_storage::sign::SignedURLMethod;
use google_cloud_storage::sign::SignedURLOptions;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use tokio::task::JoinHandle;

pub async fn upload_file() {
    // Create client.

    // read in service account file
    let service_account_file = String::from("/Users/bjar/service-account.json");

    let json_file = fs::read_to_string(service_account_file).expect("Unable to read file");
    let service_acct: CredentialsFile =
        serde_json::from_str(&json_file).expect("JSON was not well-formatted");

    let sa_box = Box::new(service_acct);
    let project = Project::FromFile(sa_box);

    let mut client = Client::new(&project, ClientConfig::default())
        .await
        .unwrap();

    println!("{:?}", client.project_id());

    let url_for_upload = client
        .signed_url(
            "aspn_functions",
            "foo.txt",
            SignedURLOptions {
                method: SignedURLMethod::PUT,
                ..Default::default()
            },
        )
        .await
        .unwrap();

    println!("{:?}", url_for_upload);
}
