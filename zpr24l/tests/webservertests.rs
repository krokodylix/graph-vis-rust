use reqwest::Client;

#[tokio::test]
async fn check_if_server_returns_200() {
    let client = Client::new();
    let res = client.get("http://localhost:8000").send().await.unwrap();
    assert_eq!(res.status(), 200);
}


#[tokio::test]
async fn check_if_server_returns_404() {
    let client = Client::new();
    let res = client.get("http://localhost:8000/does/not/exist").send().await.unwrap();
    assert_eq!(res.status(), 404);
}

#[tokio::test]
async fn check_default_api_endpoint() {
    let client = Client::new();
    let res = client.get("http://localhost:8000/api/hellozpr").send().await.unwrap();
    assert_eq!(res.status(), 200);
}