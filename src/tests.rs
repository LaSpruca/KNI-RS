use super::*;

#[tokio::test]
async fn test_today() {
    let portal = Portal::new("https://portal.kamohigh.school.nz/api/api.php");
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
    let portal = Portal::new("https://demo.school.nz/api/api.php");
    // I know this is bad but shud up.
    let mut workey = false;
    if let Ok(data) = portal.get_notices(&date).await {
        println!("{}", data);
        workey = true;
    }
    assert_eq!(workey, true);

}
