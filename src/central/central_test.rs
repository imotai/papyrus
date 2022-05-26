use super::*;
use mockito::mock;

#[tokio::test]
async fn get_block_number() {
    let central_client = CentralClient::new(&mockito::server_url()).unwrap();
    let mock = mock("GET", "/feeder_gateway/get_last_batch_id")
        .with_status(200)
        .with_body("195812")
        .create();
    let block_number = central_client.block_number().await.unwrap();
    mock.assert();
    assert_eq!(block_number, BlockNumber(195812));
}
