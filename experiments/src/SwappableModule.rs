#![allow(dead_code)]

/// Example: Swappable logging/storage backends using trait interfaces
pub mod SwappableModule {
    use std::fmt::Display;
    
    // Define the interface for a storage backend
    pub trait Storage {
        fn save(&self, key: &str, value: &str) -> Result<(), String>;
        fn load(&self, key: &str) -> Result<String, String>;
        fn delete(&self, key: &str) -> Result<(), String>;
    }
    
    // Implementation 1: In-memory storage (for testing)
    pub struct MemoryStorage {
        data: std::cell::RefCell<std::collections::HashMap<String, String>>,
    }
    
    impl MemoryStorage {
        pub fn new() -> Self {
            Self {
                data: std::cell::RefCell::new(std::collections::HashMap::new()),
            }
        }
    }
    
    impl Storage for MemoryStorage {
        fn save(&self, key: &str, value: &str) -> Result<(), String> {
            self.data.borrow_mut().insert(key.to_string(), value.to_string());
            Ok(())
        }
        
        fn load(&self, key: &str) -> Result<String, String> {
            self.data.borrow()
                .get(key)
                .map(|v| v.clone())
                .ok_or_else(|| format!("Key '{}' not found", key))
        }
        
        fn delete(&self, key: &str) -> Result<(), String> {
            self.data.borrow_mut().remove(key);
            Ok(())
        }
    }
    
    // Implementation 2: Mock storage that logs operations
    pub struct MockStorage;
    
    impl Storage for MockStorage {
        fn save(&self, key: &str, value: &str) -> Result<(), String> {
            println!("[MOCK] Saving '{}' = '{}'", key, value);
            Ok(())
        }
        
        fn load(&self, key: &str) -> Result<String, String> {
            println!("[MOCK] Loading '{}'", key);
            Ok(format!("mock_value_for_{}", key))
        }
        
        fn delete(&self, key: &str) -> Result<(), String> {
            println!("[MOCK] Deleting '{}'", key);
            Ok(())
        }
    }
    
    // Business logic that works with any Storage implementation
    pub struct UserManager<S: Storage> {
        storage: S,
    }
    
    impl<S: Storage> UserManager<S> {
        pub fn new(storage: S) -> Self {
            Self { storage }
        }
        
        pub fn create_user(&self, username: &str, email: &str) -> Result<(), String> {
            let key = format!("user:{}", username);
            self.storage.save(&key, email)
        }
        
        pub fn get_user_email(&self, username: &str) -> Result<String, String> {
            let key = format!("user:{}", username);
            self.storage.load(&key)
        }
        
        pub fn delete_user(&self, username: &str) -> Result<(), String> {
            let key = format!("user:{}", username);
            self.storage.delete(&key)
        }
    }
    
    // Convenience: Create with default implementation
    pub fn create_user_manager() -> UserManager<MemoryStorage> {
        UserManager::new(MemoryStorage::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SwappableModule::*;
    
    #[test]
    fn test_memory_storage() {
        let manager = UserManager::new(MemoryStorage::new());
        
        manager.create_user("alice", "alice@example.com").unwrap();
        let email = manager.get_user_email("alice").unwrap();
        
        assert_eq!(email, "alice@example.com");
    }
    
    #[test]
    fn test_mock_storage() {
        let manager = UserManager::new(MockStorage);
        
        manager.create_user("bob", "bob@example.com").unwrap();
        let email = manager.get_user_email("bob").unwrap();
        
        assert_eq!(email, "mock_value_for_user:bob");
    }
    
    #[test]
    fn test_default_manager() {
        let manager = create_user_manager();
        
        manager.create_user("charlie", "charlie@example.com").unwrap();
        assert_eq!(manager.get_user_email("charlie").unwrap(), "charlie@example.com");
    }
}



