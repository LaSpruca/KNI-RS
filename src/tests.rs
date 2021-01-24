use super::*;

const TEST_URL: &str = "https://demo.school.kiwi/api/api.php";

#[tokio::test]
async fn test_today() {
    let portal = Portal::new(TEST_URL);
    // I know this is bad but shud up.
    let mut workey = false;
    if let Ok(data) = portal.get_notices_today().await {
        println!("{}", data);
        workey = true;
    }
    assert_eq!(workey, true);
}

#[tokio::test]
async fn test_prev() {
    let date = parse_date("2020-11-03");
    let portal = Portal::new(TEST_URL);
    // I know this is bad but shud up.
    let request = portal.get_notices(&date).await;
    let mut workey = false;
    if let Ok(data) = request {
        println!("{}", data);
        workey = true;
    } else {
        println!("{:?}", request);
    }
    assert_eq!(workey, true);

}
