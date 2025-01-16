//! Module pour la gestion d'un cache LRU persistant.
//!
//! ## Fonctionnalités principales
//! - Ajouter des éléments au cache et les sauvegarder automatiquement.
//! - Récupérer des éléments depuis le cache en mettant à jour leur priorité.
//! - Sauvegarder l'état actuel du cache dans un fichier.
//! - Charger les données d'un fichier pour restaurer un cache persistant.

use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::str::FromStr;

/// # Structure `PersistentCache`
///
/// Cette structure représente un cache LRU persistant. Elle sauvegarde
/// automatiquement les données dans un fichier chaque fois qu'un élément
/// est ajouté ou modifié.
///
/// ## Types génériques
/// - `K` : Type des clés (doit être `Eq`, `Hash`, `Clone`, `ToString`, et `FromStr`).
/// - `V` : Type des valeurs (doit être `ToString` et `FromStr`).
#[derive(Debug)]
pub struct PersistentCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    keys: Vec<K>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V>
where
    K: Eq + std::hash::Hash + Clone + ToString + FromStr,
    V: ToString + FromStr,
    <K as FromStr>::Err: std::fmt::Debug,
    <V as FromStr>::Err: std::fmt::Debug,
{
    /// ## Méthode `new`
    ///
    /// Crée un cache avec une capacité maximale et un chemin vers un fichier
    /// de sauvegarde.
    ///
    /// ### Arguments
    /// - `capacity` : Capacité maximale du cache.
    /// - `file_path` : Chemin du fichier de sauvegarde.
    ///
    /// ### Exemple
    /// ```rust
    /// let cache: PersistentCache<String, String> = PersistentCache::new(3, "cache.txt");
    /// ```
    pub fn new(capacity: usize, file_path: &str) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            keys: Vec::new(),
            file_path: file_path.to_string(),
        }
    }

    /// ## Méthode `save`
    ///
    /// Sauvegarde les données du cache dans le fichier défini lors de la création.
    ///
    /// ### Retourne
    /// - `Ok(())` : Si la sauvegarde réussit.
    /// - `Err(io::Error)` : Si une erreur d'écriture se produit.
    ///
    /// ### Exemple
    /// ```rust
    /// cache.save().unwrap();
    /// ```
    pub fn save(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        for key in &self.keys {
            if let Some(value) = self.map.get(key) {
                writeln!(file, "{}:{}", key.to_string(), value.to_string())?;
            }
        }
        Ok(())
    }

    /// ## Méthode `load`
    ///
    /// Charge les données depuis le fichier défini lors de la création dans un cache.
    /// Les clés et valeurs doivent être parsables depuis une chaîne de caractères.
    ///
    /// ### Retourne
    /// - `Ok(())` : Si le chargement réussit.
    /// - `Err(io::Error)` : Si une erreur de lecture ou de parsing se produit.
    ///
    /// ### Exemple
    /// ```rust
    /// cache.load().unwrap();
    /// ```
    pub fn load(&mut self) -> io::Result<()> {
        let content = fs::read_to_string(&self.file_path)?;
        for line in content.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.parse::<K>().expect("Clé invalide");
                let value = value.parse::<V>().expect("Valeur invalide");
                self.put(key, value);
            }
        }
        Ok(())
    }

    /// ## Méthode `put`
    ///
    /// Ajoute une clé et une valeur au cache. Si la clé existe déjà,
    /// elle est mise à jour. Si la capacité est atteinte, le plus ancien
    /// élément est supprimé. La sauvegarde est effectuée automatiquement.
    ///
    /// ### Arguments
    /// - `key` : La clé de l'élément.
    /// - `value` : La valeur associée.
    ///
    /// ### Exemple
    /// ```rust
    /// cache.put("A".to_string(), "Valeur A".to_string());
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
        self.save().unwrap();
    }

    /// ## Méthode `get`
    ///
    /// Récupère une valeur depuis sa clé. Si la clé est trouvée, elle est
    /// marquée comme récemment utilisée.
    ///
    /// ### Arguments
    /// - `key` : La clé de l'élément à récupérer.
    ///
    /// ### Retourne
    /// - `Some(&V)` : Une référence immuable à la valeur si elle est trouvée.
    /// - `None` : Si la clé n'existe pas dans le cache.
    ///
    /// ### Exemple
    /// ```rust
    /// if let Some(value) = cache.get(&"A".to_string()) {
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
