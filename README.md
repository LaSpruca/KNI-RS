# KNI-RS
## A rust interface for getting notices from KAMAR
_____________________
__This is still WIP__
_____________________
This is an iterface for accessing the KAMAR API in rust to retrieve the notices

Example Usage:
```rust
use kni_rs::Portal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let portal = Portal::new("https://demo.school.kiwi");
    let notices = protal.get_notices_today().await?;
    
    println!("Notices: {:?}", notices);
    
    Ok(())
}
```