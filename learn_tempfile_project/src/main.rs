use error_chain::error_chain;
use reqwest::header::IF_NONE_MATCH;
use std::fs::File;
use std::io::{Cursor, copy};
use tempfile::Builder;
use tokio;
error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Creating an temporary file...");
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://fastly.picsum.photos/id/305/536/354.jpg?hmac=u9Mw37522_zEQ1FdKoVv_QXkWMkBOZFzRSXn2MCW0IY";
    let response = reqwest::get(target).await?;
    // println!("{:?}", response);
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("file to download : {}", fname);
        let fname = tmp_dir.path().join(fname);
        println!("File location : {:?}", fname);
        File::create(fname)?
    };
    let content = response.text().await?;
    let mut reader = Cursor::new(content);
    copy(&mut reader, &mut dest)?;
    Ok(())
}
