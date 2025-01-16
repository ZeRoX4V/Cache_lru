//! Module implémentant un cache LRU.
//!
//! Ce cache garde les `N` éléments les plus récemment utilisés.
//! Si la capacité est atteinte, le plus ancien élément est retiré.
//!
//! ## Fonctionnalités principales
//! - Ajouter des éléments au cache avec la méthode [`Cache::put`].
//! - Récupérer des éléments du cache avec la méthode [`Cache::get`].

use std::collections::HashMap;

/// # Structure `Cache`
///
/// Cette structure représente un cache LRU générique.
/// Elle utilise un `HashMap` pour stocker les données et un `Vec` pour maintenir l'ordre d'accès.
///
/// ## Types génériques
/// - `K` : Le type des clés (doit être `Eq`, `Hash`, et `Clone`).
/// - `V` : Le type des valeurs.
#[derive(Debug)]
pub struct Cache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    keys: Vec<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> Cache<K, V> {
    /// ## Méthode `new`
    ///
    /// Crée un nouveau cache LRU avec une capacité donnée.
    ///
    /// ### Arguments
    /// - `capacity` : La capacité maximale du cache.
    ///
    /// ### Exemple
    /// ```rust
    /// let cache: Cache<&str, String> = Cache::new(3);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    /// ## Méthode `put`
    ///
    /// Ajoute une clé et une valeur au cache. Si la clé existe déjà,
    /// elle est mise à jour. Si la capacité est atteinte, le plus ancien
    /// élément est retiré.
    ///
    /// ### Arguments
    /// - `key` : La clé associée à la valeur.
    /// - `value` : La valeur à stocker.
    ///
    /// ### Exemple
    /// ```rust
    /// let mut cache = Cache::new(2);
    /// cache.put("A", "Valeur A".to_string());
    /// ```
    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.keys.retain(|k| k != &key);
        } else if self.keys.len() >= self.capacity {
            if let Some(old_key) = self.keys.pop() {
                self.map.remove(&old_key);
            }
        }
        self.keys.insert(0, key.clone());
        self.map.insert(key, value);
    }

    /// ## Méthode `get`
    ///
    /// Récupère une valeur à partir de sa clé. Si la clé existe,
    /// elle est marquée comme récemment utilisée.
    ///
    /// ### Arguments
    /// - `key` : La clé à rechercher dans le cache.
    ///
    /// ### Retourne
    /// - `Some(&V)` : La valeur lié à la key si elle est présente.
    /// - `None` : Si la clé n'est pas présente dans le cache.
    ///
    /// ### Exemple
    /// ```rust
    /// let mut cache = Cache::new(2);
    /// cache.put("A", "Valeur A".to_string());
    /// if let Some(value) = cache.get(&"A") {
    ///     println!("Valeur : {}", value);
    /// }
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.keys.retain(|k| k != key);
            self.keys.insert(0, key.clone());
            self.map.get(key)
        } else {
            None
        }
    }
}
