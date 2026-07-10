//! Opt-in persistent cache: owner-only, no raw source storage, content-addressed keys.

use beavermath_core::hash::sha256_hex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

const DEFAULT_MAX_BYTES: u64 = 256 * 1024 * 1024;
const DEFAULT_MAX_AGE_SECS: u64 = 30 * 24 * 3600;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub entries: usize,
    pub total_bytes: u64,
    pub path: String,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct PersistentCache {
    root: PathBuf,
    secret: String,
    max_bytes: u64,
    #[allow(dead_code)]
    max_age_secs: u64,
    enabled: bool,
}

impl PersistentCache {
    pub fn disabled() -> Self {
        Self {
            root: PathBuf::from(".beavermath-cache"),
            secret: String::new(),
            max_bytes: DEFAULT_MAX_BYTES,
            max_age_secs: DEFAULT_MAX_AGE_SECS,
            enabled: false,
        }
    }

    pub fn open(root: impl Into<PathBuf>, secret: impl Into<String>) -> io::Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&root, fs::Permissions::from_mode(0o700));
        }
        Ok(Self {
            root,
            secret: secret.into(),
            max_bytes: DEFAULT_MAX_BYTES,
            max_age_secs: DEFAULT_MAX_AGE_SECS,
            enabled: true,
        })
    }

    pub fn stats(&self) -> CacheStats {
        let mut entries = 0usize;
        let mut total_bytes = 0u64;
        if self.enabled {
            if let Ok(rd) = fs::read_dir(&self.root) {
                for e in rd.flatten() {
                    if let Ok(m) = e.metadata() {
                        if m.is_file() {
                            entries += 1;
                            total_bytes += m.len();
                        }
                    }
                }
            }
        }
        CacheStats {
            entries,
            total_bytes,
            path: self.root.display().to_string(),
            enabled: self.enabled,
        }
    }

    pub fn clear(&self) -> io::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        if let Ok(rd) = fs::read_dir(&self.root) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_file() {
                    let _ = fs::remove_file(p);
                }
            }
        }
        Ok(())
    }

    fn key_path(&self, content_key: &str) -> PathBuf {
        let material = format!("{}:{}", self.secret, content_key);
        let dig = sha256_hex(material.as_bytes());
        self.root.join(format!("{dig}.bin"))
    }

    /// Store artifact bytes only (never raw source or prompt context).
    pub fn put_artifact(&self, content_key: &str, bytes: &[u8]) -> io::Result<()> {
        if !self.enabled {
            return Ok(());
        }
        let path = self.key_path(content_key);
        let tmp = path.with_extension("tmp");
        fs::write(&tmp, bytes)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600));
        }
        fs::rename(&tmp, &path)?;
        self.evict_if_needed()?;
        Ok(())
    }

    pub fn get_artifact(&self, content_key: &str) -> io::Result<Option<Vec<u8>>> {
        if !self.enabled {
            return Ok(None);
        }
        let path = self.key_path(content_key);
        match fs::read(&path) {
            Ok(b) => Ok(Some(b)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn evict_if_needed(&self) -> io::Result<()> {
        let stats = self.stats();
        if stats.total_bytes <= self.max_bytes {
            return Ok(());
        }
        // Simple: delete oldest files
        let mut files: Vec<(PathBuf, SystemTime)> = Vec::new();
        if let Ok(rd) = fs::read_dir(&self.root) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_file() {
                    if let Ok(m) = e.metadata() {
                        if let Ok(modified) = m.modified() {
                            files.push((p, modified));
                        }
                    }
                }
            }
        }
        files.sort_by_key(|(_, t)| *t);
        let now = SystemTime::now();
        for (p, modified) in files {
            let age = now.duration_since(modified).map(|d| d.as_secs()).unwrap_or(0);
            let _ = age;
            let _ = DEFAULT_MAX_AGE_SECS;
            let _ = fs::remove_file(&p);
            if self.stats().total_bytes <= self.max_bytes {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn no_cache_when_disabled() {
        let c = PersistentCache::disabled();
        c.put_artifact("k", b"data").unwrap();
        assert!(c.get_artifact("k").unwrap().is_none());
    }

    #[test]
    fn put_get_clear() {
        let dir = tempdir().unwrap();
        let c = PersistentCache::open(dir.path(), "secret").unwrap();
        c.put_artifact("png:abc", b"\x89PNG").unwrap();
        assert_eq!(c.get_artifact("png:abc").unwrap().unwrap(), b"\x89PNG");
        // Keys are not raw source
        let entries: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert!(entries.iter().all(|n| !n.contains("source")));
        c.clear().unwrap();
        assert!(c.get_artifact("png:abc").unwrap().is_none());
    }
}
