use std::path::{Path, PathBuf};

use crate::Result;

pub mod sync {
    use std::path::Path;

    use crate::Result;
    // use super::get_available_filename;

    use tokio::{fs::File, io::AsyncWriteExt};

    pub async fn overwrite<T>(content: T, path: &Path) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let mut file = File::create(&path).await?;
    
        file.write_all(content.as_ref()).await?;
        file.flush().await.map_err(Into::into)
    }
    
    // pub async fn new<T>(content: T, path: &Path) -> Result<()>
    // where
    //     T: AsRef<[u8]>,
    // {
    //     let valid_path = if path.exists() {
    //         get_available_filename(path)?
    //     } else {
    //         path.to_path_buf()
    //     };
    
    //     let mut file = File::create(&valid_path).await?;
    
    //     file.write_all(content.as_ref()).await?;
    //     file.flush().await.map_err(Into::into)
    // }
}

pub mod blocking {

    use std::{fs::File, io::Write, path::Path};

    use crate::Result;
    use super::get_available_filename;

    pub fn new<T>(content: T, path: &Path) -> Result<()>
    where
        T: AsRef<[u8]>,
    {
        let valid_path = if path.exists() {
            get_available_filename(path)?
        } else {
            path.to_path_buf()
        };
    
        let mut file = File::create(&valid_path)?;
    
        file.write_all(content.as_ref())?;
        file.flush().map_err(Into::into)
    }  
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
