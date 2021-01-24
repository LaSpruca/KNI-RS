use super::*;

const TEST_URL: &str = "https://demo.school.kiwi/api/api.php";

#[tokio::test]
async fn test_today() {
    let portal = Portal::new(TEST_URL);
    let result = portal.get_notices_today().await;
    match result {
        Ok(val) => {
            assert_eq!(val.error_code, 0)
        },
        Err(e) => {
            assert_ne!(e.error_code, 0)
        }
    }
}

#[tokio::test]
async fn test_prev() {
    let date = parse_date("2020-11-03");
    let portal = Portal::new(TEST_URL);
    let result = portal.get_notices(&date).await;
    match result {
        Ok(val) => {
            assert_eq!(val.error_code, 0)
        },
        Err(e) => {
            assert_ne!(e.error_code, 0)
        }
    }
}

#[tokio::test]
async fn test_bad_key() {
    let portal = Portal::with_key(TEST_URL, "e");
    let result = portal.get_notices_today().await;
    match result {
        Ok(val) => {
            assert_eq!(val.error_code, 0)
        },
        Err(e) => {
            assert_ne!(e.error_code, 0)
        }
    }
}
