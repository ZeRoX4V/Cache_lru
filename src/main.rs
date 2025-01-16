// # Démonstration du Cache LRU
//
// Ce programme illustre l'utilisation d'un cache LRU en deux versions :
// - **Cache non persistant** : Les données sont stockées uniquement en mémoire.
//  - **Cache persistant** : Les données sont sauvegardées dans un fichier texte pour être restaurées ultérieurement.
//
//
// ## Résumé de l'exécution
// - Initialisation d'un cache non persistant, ajout et récupération de données.
// - Initialisation d'un cache persistant, ajout, sauvegarde, et rechargement depuis un fichier.

mod cache_lru;
mod storage;

use crate::cache_lru::Cache;
use crate::storage::PersistentCache;

fn main() {
    println!("--------------------------------------------------------- Utilisation du Cache LRU  ---------------------------------------------------------");
    println!("=== Utilisation du Cache LRU Non Persistant ===");

    // On initialise le cache LRU non persistant
    let mut cache = Cache::new(3);
    println!("On initialise le cache LRU avec une capacité de 3");

    // On ajoute des données au cache
    cache.put("A".to_string(), "Valeur A".to_string());
    println!("A a été Ajouté avec pour valeur : Valeur A");
    cache.put("B".to_string(), "Valeur B".to_string());
    println!("B a été Ajouté avec pour valeur : Valeur B");
    cache.put("C".to_string(), "Valeur C".to_string());
    println!("C a été Ajouté avec pour valeur : Valeur C");

    // On récupère une valeur dans le cache LRU dans ce cas la valeur A
    if let Some(value) = cache.get(&"A".to_string()) {
        println!(
            "Valeur récupéré pour la clé A dans le cache LRU : -> {}",
            value
        );
    } else {
        println!("La clé A n'a pas été trouvée dans le cache");
    }

    // On ajoute une 4ème valeur dans le cache
    cache.put("D".to_string(), "Valeur D".to_string());
    println!("Ajouté : D -> Valeur D, la valeur B a été supprimé du cache");

    // On affiche le contenu du cache
    println!("Contenu du cache LRU : {:?}", cache);

    // Utilisation du cache LRU persistant
    println!("\n---------------------------------------------");
    println!("\n=== Utilisation du Cache LRU Persistant ===");

    // On initialise le cache LRU persistant
    let mut persistent_cache = PersistentCache::new(3, "cache_persistant.txt");
    println!("On initialise le cache LRU avec une capacité de 3");

    // On ajoute des données au cache
    persistent_cache.put("E".to_string(), "Valeur E".to_string());
    println!("E a été Ajouté avec pour valeur : Valeur E");
    persistent_cache.put("F".to_string(), "Valeur F".to_string());
    println!("F a été Ajouté avec pour valeur : Valeur F");
    persistent_cache.put("G".to_string(), "Valeur G".to_string());
    println!("G a été Ajouté avec pour valeur : Valeur G");

    // On sauvegarde le contenu du cache dans un fichier.txt
    println!("Sauvegarde du cache dans 'cache_persistent.txt'.");
    persistent_cache.save().unwrap();

    // On récupère une valeur dans le cache LRU dans ce cas la valeur E
    if let Some(value) = persistent_cache.get(&"E".to_string()) {
        println!(
            "Valeur récupéré pour la clé E dans le cache LRU : -> {}",
            value
        );
    } else {
        println!("La clé E n'a pas été trouvée dans le cache");
    }

    // On ajoute une 4ème valeur dans le cache
    persistent_cache.put("H".to_string(), "Valeur H".to_string());
    println!("Ajouté : H -> Valeur H, la valeur F a été supprimé du cache");

    // On remplis le cache avec le contenu du fichier txt
    println!("On charge le cache depuis le fichier : cache_persistent.txt");
    persistent_cache.load().unwrap();

    // On affiche le contenu du cache
    println!("Contenu du cache LRU : {:?}", persistent_cache);
}
