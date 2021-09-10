#[cfg(test)]
pub mod utils {

    use actix_web::{dev::Body, HttpResponse};
    use std::path::Path;
    use tempdir::TempDir;

    use crate::{config::Config, utils::dirs, Result};

    pub const TEST_DEV: &str = "1234567890123456tPKISeTp5Pm9B4L3SpFRask0biEpRTssHW4/7e7ZFhc=";

    pub fn create_test_folder(path: &Path) -> Result<TempDir> {
        TempDir::new(&format!("rusko-test-{}", path.display())).map_err(Into::into)
    }

    pub fn create_test_config(path: &Path) -> Result<Config> {
        let dirs = dirs::directories(Some(&path.to_path_buf()))?;
        Config::new(dirs)
    }

    pub async fn create_test_config_with_paired_device(path: &Path, dev: &str) -> Result<Config> {
        let mut config = create_test_config(path)?;
        config.pair_device(dev.into()).await?;
        Ok(config)
    }

    pub fn res_body_to_string(res: HttpResponse) -> Result<String> {
        let body = res.body();
        if let Body::Bytes(b) = body {
            return String::from_utf8(b.to_vec()).map_err(Into::into);
        }
        Err("Failed to read body".into())
    }

    #[test]
    fn temp_folder_exists() -> Result<()> {
        let tempdir = create_test_folder(Path::new("temp"))?;
        assert!(tempdir.path().exists());
        tempdir.close()?;
        Ok(())
    }
}
