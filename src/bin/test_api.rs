use aspn::utils::api::{
    models::NewFunction,
    storage::function::{find, find_by_project, save},
};

#[tokio::main]
async fn main() {
    let nf = NewFunction {
        project_id: 1,
        route: "my/test/route".to_string(),
        gcs_uri: "fake/gcs/uri".to_string(),
    };

    let res = save(&nf).await.unwrap();
    let res = find(&res.id).await.unwrap();
    let res = find_by_project(&res.project_id).await.unwrap();
    println!("{:?}", res);
}
