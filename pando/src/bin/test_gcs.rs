use pando::storage;

fn main() {
    storage::gcs::upload_file();
    storage::gcs::download_file();
}
