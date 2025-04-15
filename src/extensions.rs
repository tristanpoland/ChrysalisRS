use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::error::{Result, Error};

/// Trait for extensions to ChrysalisRS
pub trait Extension: Send + Sync {
    /// Get the name of the extension
    fn name(&self) -> &str;
    
    /// Initialize the extension
    fn initialize(&mut self) -> Result<()>;
    
    /// Shutdown the extension
    fn shutdown(&mut self) -> Result<()>;
    
    /// Check if the extension is enabled
    fn is_enabled(&self) -> bool;
    
    /// Enable or disable the extension
    fn set_enabled(&mut self, enabled: bool);
    
    /// Get extension as Any for downcasting
    fn as_any(&self) -> &dyn Any;
    
    /// Get mutable extension as Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Registry for managing extensions
pub struct ExtensionRegistry {
    extensions: HashMap<String, Box<dyn Extension>>,
    type_map: HashMap<TypeId, String>,
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            type_map: HashMap::new(),
        }
    }
    
    /// Register an extension
    pub fn register<E: Extension + 'static>(&mut self, extension: E) -> Result<()> {
        let name = extension.name().to_string();
        let type_id = TypeId::of::<E>();
        
        if self.extensions.contains_key(&name) {
            return Err(Error::ExtensionError(format!(
                "Extension with name '{}' is already registered", name
            )));
        }
        
        self.type_map.insert(type_id, name.clone());
        self.extensions.insert(name, Box::new(extension));
        Ok(())
    }
    
    /// Get an extension by name
    pub fn get(&self, name: &str) -> Option<&dyn Extension> {
        self.extensions.get(name).map(|ext| ext.as_ref())
    }
    
    /// Get a mutable extension by name
    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Extension> {
        if let Some(ext) = self.extensions.get_mut(name) {
            Some(&mut **ext)
        } else {
            None
        }
    }
    
    /// Get an extension by type
    pub fn get_by_type<E: Extension + 'static>(&self) -> Option<&E> {
        let type_id = TypeId::of::<E>();
        
        if let Some(name) = self.type_map.get(&type_id) {
            if let Some(ext) = self.extensions.get(name) {
                return ext.as_any().downcast_ref::<E>();
            }
        }
        
        None
    }
    
    /// Get a mutable extension by type
    pub fn get_mut_by_type<E: Extension + 'static>(&mut self) -> Option<&mut E> {
        let type_id = TypeId::of::<E>();
        
        if let Some(name) = self.type_map.get(&type_id).cloned() {
            if let Some(ext) = self.extensions.get_mut(&name) {
                return ext.as_any_mut().downcast_mut::<E>();
            }
        }
        
        None
    }
    
    /// Remove an extension by name
    pub fn remove(&mut self, name: &str) -> Option<Box<dyn Extension>> {
        self.extensions.remove(name)
    }
    
    /// Initialize all extensions
    pub fn initialize_all(&mut self) -> Result<()> {
        for (name, ext) in &mut self.extensions {
            if let Err(e) = ext.initialize() {
                return Err(Error::ExtensionError(format!(
                    "Failed to initialize extension '{}': {}", name, e
                )));
            }
        }
        Ok(())
    }
    
    /// Shutdown all extensions
    pub fn shutdown_all(&mut self) -> Result<()> {
        for (name, ext) in &mut self.extensions {
            if let Err(e) = ext.shutdown() {
                return Err(Error::ExtensionError(format!(
                    "Failed to shutdown extension '{}': {}", name, e
                )));
            }
        }
        Ok(())
    }
}