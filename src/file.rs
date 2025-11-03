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
        let pipeline_description = format!(
            "playbin uri=\"file://{}\" video-sink=\"videoconvert\"",
            self.source.to_string_lossy()
        );
        VideoStream::new(pipeline_description).into_iter()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
}
