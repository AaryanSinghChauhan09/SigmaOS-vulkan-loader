// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-vulkan-loader: vulkan loader component model.
//!
//! Real implementation: loaders, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loader {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VulkanLoaderError {
    EmptyLoaderName,
    DuplicateLoader(String),
    EmptyEventSubject,
}

impl fmt::Display for VulkanLoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLoaderName => write!(f, "loader name must not be empty"),
            Self::DuplicateLoader(id) => write!(f, "duplicate loader {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VulkanLoaderError {}

pub struct VulkanLoader {
    pub name: String,
    loaders: BTreeMap<String, Loader>,
    events: BTreeMap<String, Event>,
}

impl VulkanLoader {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            loaders: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_loader(&mut self, item: Loader) -> Result<(), VulkanLoaderError> {
        if item.name.trim().is_empty() {
            return Err(VulkanLoaderError::EmptyLoaderName);
        }
        let id = item.id.clone();
        if self.loaders.contains_key(&id) {
            return Err(VulkanLoaderError::DuplicateLoader(id.clone()));
        }
        self.loaders.insert(id, item);
        Ok(())
    }

    pub fn loader_count(&self) -> usize {
        self.loaders.len()
    }

    pub fn get_loader(&self, id: &str) -> Option<&Loader> {
        self.loaders.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VulkanLoaderError> {
        if event.subject.trim().is_empty() {
            return Err(VulkanLoaderError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_loader(id: &str, name: &str) -> Loader {
        Loader {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = VulkanLoader::new("VulkanLoader");
        assert_eq!(m.name, "VulkanLoader");
        assert_eq!(m.loader_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_loader() {
        let mut m = VulkanLoader::new("VulkanLoader");
        m.add_loader(sample_loader("a1", "Alpha")).unwrap();
        assert_eq!(m.loader_count(), 1);
        assert!(m.get_loader("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = VulkanLoader::new("VulkanLoader");
        assert!(m.add_loader(sample_loader("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = VulkanLoader::new("VulkanLoader");
        m.add_loader(sample_loader("x", "One")).unwrap();
        assert!(m.add_loader(sample_loader("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = VulkanLoader::new("VulkanLoader");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
