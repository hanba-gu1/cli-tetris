use std::{io::Result, path::Path};

use tokio::io::AsyncWriteExt;

pub struct Logger {
    file: tokio::io::BufWriter<tokio::fs::File>,
}
impl Logger {
    pub async fn new(file_path: &Path) -> Result<Self> {
        if !tokio::fs::try_exists(file_path).await? {
            tokio::fs::write(file_path, "").await?;
        }
        let file = tokio::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .await?;
        let file = tokio::io::BufWriter::new(file);
        Ok(Logger { file })
    }
    pub async fn log<T: AsRef<[u8]>>(&mut self, text: T) -> Result<()> {
        self.file.write(text.as_ref()).await?;
        self.file.write(b"\n").await?;
        self.file.flush().await?;
        Ok(())
    }
}
