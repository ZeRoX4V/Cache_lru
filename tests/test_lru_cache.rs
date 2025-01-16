use cache_lru::cache_lru::Cache;

#[cfg(test)]
mod tests {
    use super::*;

    // On test le cachr LRU sans persistance avec des valeurs de type string et int
    #[test]
    fn test_lru_cache() {
        // On test le cachr LRU sans persistance avec des valeurs de type string
        println!("=== On effectue un premier test avec des valeurs de type string ===");

        // On créé un nouveau cache
        let mut cache = Cache::new(3);
        println!("Cache initialisé avec une capacité de 3.");

        // On ajoute des valeurs
        cache.put("A", String::from("value_a"));
        println!("Ajouté : A -> value_a");
        cache.put("B", String::from("value_b"));
        println!("Ajouté : B -> value_b");
        cache.put("C", String::from("value_c"));
        println!("Ajouté : C -> value_c");
        cache.put("D", String::from("value_d"));
        println!("Ajouté : D -> value_d, A a été suprimé");

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache);
        println!("----------------------------------");

        // Vérification des valeurs
        println!("\n--- On vérifie si des valeurs sont présente ou non dans le cache : ---");
        let my_value = cache.get(&"A");
        println!("Recherche de la clé A : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache.get(&"D");
        println!("Recherche de la clé D : {:?}", my_value);
        assert_eq!(my_value, Some(&String::from("value_d")));

        let my_value = cache.get(&"B");
        println!("Recherche de la clé B : {:?}", my_value);
        assert_eq!(my_value, Some(&String::from("value_b")));

        let my_value = cache.get(&"C");
        println!("Recherche de la clé C : {:?}", my_value);
        assert_eq!(my_value, Some(&String::from("value_c")));

        let my_value = cache.get(&"X");
        println!("Recherche de la clé X : {:?}", my_value);
        assert_eq!(my_value, None);

        // On met à jour le cache
        println!("\n--- Mise à jour du cache : ---");
        cache.put("A", String::from("value_a"));
        println!("Ajouté : A -> value_a");
        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache);
        println!("----------------------------------");

        cache.put("X", String::from("value_x"));
        println!("Ajouté : X -> value_x, B a été supprimé");
        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache);
        println!("----------------------------------");

        // Vérification après mise à jour
        let my_value = cache.get(&"B");
        println!("Recherche de la clé B après mise à jour : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache.get(&"D");
        println!("Recherche de la clé D après mise à jour : {:?}", my_value);
        assert_eq!(my_value, None);

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache);
        println!("----------------------------------");

        println!("=== Fin du test avec des valeurs de type string ===");

        println!("\n");
        println!("\n////////////////////////////////////////////////////////////////");
        println!("\n");

        // On test le cachr LRU sans persistance avec des valeurs de type int
        println!("=== On effectue un second test avec des valeurs de type int ===");

        // On créé un nouveau cache
        let mut cache_int = Cache::new(3);
        println!("Cache initialisé avec une capacité de 3.");

        // On ajoute des valeurs
        cache_int.put(1, 100);
        println!("Ajouté : 1 -> 100");
        cache_int.put(2, 200);
        println!("Ajouté : 2 -> 200");
        cache_int.put(3, 300);
        println!("Ajouté : 3 -> 300");
        cache_int.put(4, 400);
        println!("Ajouté : 4 -> 400, 1 a été supprimé du cache");

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_int);
        println!("----------------------------------");

        // Vérification des valeurs
        println!("\n--- On vérifie si des valeurs sont présente ou non dans le cache : ---");
        let my_value = cache_int.get(&1);
        println!("Recherche de la clé 1 : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache_int.get(&4);
        println!("Recherche de la clé 4 : {:?}", my_value);
        assert_eq!(my_value, Some(&400));

        let my_value = cache_int.get(&2);
        println!("Recherche de la clé 2 : {:?}", my_value);
        assert_eq!(my_value, Some(&200));

        let my_value = cache_int.get(&3);
        println!("Recherche de la clé 3 : {:?}", my_value);
        assert_eq!(my_value, Some(&300));

        // On met à jour le cache
        println!("\n--- Mise à jour du cache ---");
        cache_int.put(5, 500);
        println!("Ajouté : 5 -> 500");
        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_int);
        println!("----------------------------------");

        let my_value = cache_int.get(&4);
        println!("Recherche de la clé 4 : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache_int.get(&2);
        println!("Recherche de la clé 2 : {:?}", my_value);
        assert_eq!(my_value, Some(&200));

        let my_value = cache_int.get(&5);
        println!("Recherche de la clé 5 : {:?}", my_value);
        assert_eq!(my_value, Some(&500));

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_int);
        println!("----------------------------------");

        println!("=== Fin du test avec des valeurs de type int ===");
    }
}
