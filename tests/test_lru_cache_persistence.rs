use cache_lru::storage::PersistentCache;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    // On test le cachr LRU avec persistance avec des valeurs de type string et int
    #[test]
    fn test_lru_cache_persistent() {
        println!("=== On effectue un premier test avec des valeurs de type string ===");

        // On créé le fichier de sauvegarde
        let cache_file = "test_cache_string.txt";
        println!("Le fichier utilisé est le suivant : {}", cache_file);

        // On supprime le fichier pour que les tests soit propre au début du test
        if fs::remove_file(cache_file).is_ok() {
            println!("Fichier précédent '{}' supprimé avec succès.", cache_file);
        } else {
            println!("Aucun fichier précédent '{}' trouvé.", cache_file);
        }

        // On créé un nouveau cache
        let mut cache = PersistentCache::new(3, cache_file);
        println!("Cache initialisé avec une capacité de 3.");

        // On ajoute des valeurs
        cache.put("A".to_string(), "value_a".to_string());
        println!("Ajouté : A -> value_a");
        cache.put("B".to_string(), "value_b".to_string());
        println!("Ajouté : B -> value_b");
        cache.put("C".to_string(), "value_c".to_string());
        println!("Ajouté : C -> value_c");
        cache.put("D".to_string(), "value_d".to_string());
        println!("Ajouté : D -> value_d, A a été supprimé)");

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache);
        println!("----------------------------------");

        // Vérification des valeurs
        println!("\n--- On vérifie si des valeurs sont présente ou non dans le cache : ---");
        let my_value = cache.get(&"A".to_string());
        println!("Recherche de la clé A : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache.get(&"C".to_string());
        println!("Recherche de la clé C : {:?}", my_value);
        assert_eq!(my_value, Some(&"value_c".to_string()));

        // On sauvegarde le contenu du cache dans le fichier
        println!("\n--- Sauvegarde du contenu du cache dans le fichier ---");
        cache.save().unwrap();
        println!(
            "Le contenu du cache a bien été sauvegardé dans le fichier '{}'",
            cache_file
        );

        // On crée un nouveau cache qu'on remplit avec les valeurs stockées dans le fichier
        println!("\n---Création d'un nouveau cache remplit avec le contenu du fichier ---");
        let mut cache_charge = PersistentCache::new(3, cache_file);
        cache_charge.load().unwrap();
        println!("Contenu du cache chargé depuis le fichier '{}'", cache_file);

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_charge);
        println!("----------------------------------");

        // On vérifie que les données sont bien restaurées
        println!("\n--- Vérification que les données sont bien restaurées ---");

        assert_eq!(
            cache_charge.get(&"B".to_string()),
            Some(&"value_b".to_string())
        );
        println!("La clé B est présente dans le cache");

        assert_eq!(
            cache_charge.get(&"D".to_string()),
            Some(&"value_d".to_string())
        );
        println!("La clé D est présente dans le cache");

        assert_eq!(cache_charge.get(&"A".to_string()), None);
        println!("La clé A n'est pas dans le cache");

        // On supprime le fichier pour nettoyer après le test
        println!("\n--- Nettoyage après les tests ---");
        if fs::remove_file(cache_file).is_ok() {
            println!("Fichier '{}' supprimé après le test", cache_file);
        } else {
            println!("Impossible de supprimer le fichier '{}'", cache_file);
        }

        println!("=== Fin du test avec des valeurs de type string ===");

        println!("\n");
        println!("\n////////////////////////////////////////////////////////////////");
        println!("\n");

        // On test le cachr LRU avec persistance avec des valeurs de type int
        println!("=== On effectue un second test avec des valeurs de type int ===");

        // On créé le fichier de sauvegarde
        let cache_file_int = "test_cache_int.txt";
        println!("Le fichier utilisé est le suivant : {}", cache_file_int);

        // On supprime le fichier pour que les tests soit propre au début du test
        if fs::remove_file(cache_file_int).is_ok() {
            println!(
                "Fichier précédent '{}' supprimé avec succès",
                cache_file_int
            );
        } else {
            println!("Aucun fichier précédent '{}' trouvé", cache_file_int);
        }

        // On créé un nouveau cache
        let mut cache_int = PersistentCache::new(2, cache_file_int);
        println!("Cache initialisé avec une capacité de 2.");

        // On ajoute des valeurs
        cache_int.put(1, 100);
        println!("Ajouté : 1 -> 100");
        cache_int.put(2, 200);
        println!("Ajouté : 2 -> 200");
        cache_int.put(3, 300);
        println!("Ajouté : 3 -> 300, 1 a été supprimé");

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_int);
        println!("----------------------------------");

        // Vérification des valeurs
        println!("\n--- On vérifie si des valeurs sont présente ou non dans le cache : ---");
        let my_value = cache_int.get(&1);
        println!("Recherche de la clé 1 : {:?}", my_value);
        assert_eq!(my_value, None);

        let my_value = cache_int.get(&2);
        println!("Recherche de la clé 2 : {:?}", my_value);
        assert_eq!(my_value, Some(&200));

        let my_value = cache_int.get(&3);
        println!("Recherche de la clé 3 : {:?}", my_value);
        assert_eq!(my_value, Some(&300));

        // On sauvegarde le contenu du cache dans le fichier
        println!("\n--- Sauvegarde du contenu du cache dans le fichier ---");
        cache_int.save().unwrap();
        println!(
            "Le contenu du cache a bien été sauvegardé dans le fichier '{}'",
            cache_file_int
        );

        // On crée un nouveau cache qu'on remplit avec les valeurs stockées dans le fichier
        println!("\n---Création d'un nouveau cache remplit avec le contenu du fichier ---");
        let mut cache_charge_int = PersistentCache::new(2, cache_file_int);
        cache_charge_int.load().unwrap();
        println!(
            "Contenu du cache chargé depuis le fichier '{}'",
            cache_file_int
        );

        println!("----------------------------------");
        // On affiche le contenu du cache
        println!("Contenu du cache LRU : {:?}", cache_charge_int);
        println!("----------------------------------");

        // On vérifie que les données sont bien restaurées
        println!("\n--- Vérification que les données sont bien restaurées ---");
        assert_eq!(cache_charge_int.get(&2), Some(&200));
        println!("La clé 2 est présente dans le cache");
        assert_eq!(cache_charge_int.get(&3), Some(&300));
        println!("La clé 3 est présente dans le cache");
        assert_eq!(cache_charge_int.get(&1), None); // 1 n'existe pas
        println!("La clé 1 n'est pas dans le cache");

        // On supprime le fichier pour nettoyer après le test
        println!("\n--- Nettoyage après les tests ---");
        if fs::remove_file(cache_file_int).is_ok() {
            println!("Fichier '{}' supprimé après le test", cache_file_int);
        } else {
            println!("Impossible de supprimer le fichier '{}'", cache_file_int);
        }

        println!("=== Fin du test avec des valeurs de type int ===");
    }
}
