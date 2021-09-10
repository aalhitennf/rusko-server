use std::path::{Path, PathBuf};

use tokio::{fs::File, io::AsyncWriteExt};

use crate::Result;

pub async fn async_overwrite<T>(content: T, path: &Path) -> Result<()>
where
    T: AsRef<[u8]>,
{
    let mut file = File::create(&path).await?;

    file.write_all(content.as_ref()).await?;
    file.flush().await.map_err(Into::into)
}

pub async fn async_new<T>(content: T, path: &Path) -> Result<()>
where
    T: AsRef<[u8]>,
{
    let valid_path = if path.exists() {
        get_available_filename(path)?
    } else {
        path.to_path_buf()
    };

    let mut file = File::create(&valid_path).await?;

    file.write_all(content.as_ref()).await?;
    file.flush().await.map_err(Into::into)
}

fn get_available_filename(path: &Path) -> Result<PathBuf> {
    let base = path.parent().unwrap();
    let filename = path.file_name().unwrap().to_str().unwrap();

    let mut i = 1;

    loop {
        let try_path = base.join(format!("({}) {}", i, filename));
        if !try_path.exists() {
            return Ok(try_path);
        }
        i += 1;
        // ? Fail-safe for infinte loop (which shouldn't be possible) ?
    }
}
