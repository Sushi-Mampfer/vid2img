use crate::StreamError;
use crate::{FrameData, VideoStream, VideoStreamIterator};
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub struct FileSource {
    source: PathBuf,
}

impl FileSource {
    pub fn new(source: &Path) -> Result<Self, CaptureError> {
        let source = fs::canonicalize(source)?;
        if !source.exists() {
            return Err(CaptureError::IoError(io::Error::new(
                ErrorKind::NotFound,
                "File not found",
            )));
        }
        Ok(Self { source })
    }
}

impl IntoIterator for FileSource {
    type Item = Result<Option<FrameData>, StreamError>;
    type IntoIter = VideoStreamIterator;

    fn into_iter(self) -> Self::IntoIter {
        // Convert Windows backslashes to forward slashes and build a filesrc-based
        // pipeline. Using `filesrc ! decodebin` is more portable on Windows where
        // constructing a proper file:// URI can be error-prone.
        let path_str = self.source.to_string_lossy().replace("\\", "/");
        let pipeline_description = format!(
            "filesrc location=\"{}\" ! decodebin ! videoconvert ! video/x-raw,format=RGB",
            path_str
        );
        VideoStream::new(pipeline_description).into_iter()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
