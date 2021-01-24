<h1 style="text-align: center">KNI-RS</h1>
<h6 style="text-align: center">A rust interface for getting notices from KAMAR </h6>
<div style="width: fit-content; margin: auto; display: flex; flex-direction: row;">
    <a style="padding: 10px;">
        <img src="https://www.travis-ci.com/LaSpruca/KNI-RS.svg?branch=master&amp;status=passed" alt="build:passed">
    </a>
    <a href="https://docs.rs/kni-rs" style="padding: 10px;">
        <img src="https://docs.rs/kni-rs/badge.svg" alt="Crates.io">
    </a>
</div>

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