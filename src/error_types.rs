use core::fmt;
use core::fmt::Debug;

pub enum XpsError {
  StreamNotOpened,
  InvalidHeader,
  Unknown,
  FileNotLoaded,
  PathGetParent,
  PathToStr,
  MeshReadAscii,
  MeshReadBin,
  None,
}

impl Default for XpsError {
  fn default() -> XpsError {
    XpsError::Unknown
  }
}

impl Debug for XpsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      XpsError::StreamNotOpened => write!(f, "StreamNotOpened"),
      XpsError::InvalidHeader => write!(f, "InvalidHeader"),
      XpsError::FileNotLoaded => write!(f, "FileNotLoaded"),
      XpsError::PathGetParent => write!(f, "PathGetParent"),
      XpsError::PathToStr => write!(f, "PathToStr"),
      XpsError::MeshReadAscii => write!(f, "MeshReadAscii"),
      XpsError::MeshReadBin => write!(f, "MeshReadBin"),
      XpsError::Unknown => write!(f, "Unknown"),
      XpsError::None => write!(f, "None"),
    }
  }
}
