use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Copy)]
enum PokemonType {
    Feu,
    Eau,
    Plante,
    Electrik,
    Tenebre,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PokemonGender {
    Male,
    Femelle,
}

#[derive(Debug, Clone)]
struct Pokemon {
    id: Uuid,
    nom: String,
    niveau: u32,
    type_pkm: PokemonType,
    experience: u32,
    genre: PokemonGender,
}

impl Pokemon {
    fn new(nom: String, type_pkm: PokemonType, genre: PokemonGender) -> Self {
        Self {
            id: Uuid::new_v4(),
            nom,
            niveau: 1,
            type_pkm,
            experience: 0,
            genre,
        }
    }

    fn gagner_experience(&mut self, xp: u32) {
        self.experience += xp;
        while self.experience >= 100 {
            self.niveau += 1;
            self.experience -= 100;
        }
    }

    fn afficher(&self) {
        println!("─────────────────────────");
        println!("Informations Pokémon :");
        println!("ID: {}", self.id);
        println!("Nom: {}", self.nom);
        println!("Niveau: {}", self.niveau);
        println!("Type: {:?}", self.type_pkm);
        println!("XP: {}", self.experience);
        println!("Genre: {:?}", self.genre);
        println!("─────────────────────────");
    }
}

struct Nursery {
    pokemons: Vec<Pokemon>,
}

impl Nursery {
    fn new() -> Self {
        Self {
            pokemons: Vec::new(),
        }
    }

    fn ajouter_pokemon(&mut self, pkm: Pokemon) {
        self.pokemons.push(pkm);
    }

    fn afficher_tous(&self) {
        if self.pokemons.is_empty() {
            println!("Aucun Pokémon pour l’instant !");
            return;
        }
        for pkm in &self.pokemons {
            pkm.afficher();
        }
    }

    fn entrainer_tous(&mut self, xp: u32) {
        if self.pokemons.is_empty() {
            println!("Pas de Pokémon à entraîner !");
            return;
        }
        for pkm in &mut self.pokemons {
            pkm.gagner_experience(xp);
        }
    }

    fn peut_se_reproduire(p1: &Pokemon, p2: &Pokemon) -> bool {
        p1.type_pkm == p2.type_pkm
            && p1.genre != p2.genre
            && p1.niveau >= 5
            && p2.niveau >= 5
    }

    fn reproduire(&mut self, p1: &Pokemon, p2: &Pokemon) {
        if !Self::peut_se_reproduire(p1, p2) {
            println!("Reproduction impossible (conditions non remplies).");
            return;
        }

        let new_gender = if rand::thread_rng().gen_bool(0.5) {
            PokemonGender::Male
        } else {
            PokemonGender::Femelle
        };

        let possible_names = [
            "Pikachu", "Bulbizarre", "Salamèche", "Carapuce",
            "Évoli", "Miaouss", "Rondoudou", "Psykokwak",
            "Rattata", "Grolet", "Goupix", "Nosferapti"
        ];
        let random_name = possible_names[rand::thread_rng().gen_range(0..possible_names.len())]
            .to_string();

        let bebe = Pokemon::new(random_name, p1.type_pkm, new_gender);
        println!("Un nouveau Pokémon est apparu !");
        bebe.afficher();
        self.ajouter_pokemon(bebe);
    }

    fn tri_par_niveau(&mut self) {
        self.pokemons.sort_by(|a, b| b.niveau.cmp(&a.niveau));
    }

    fn tri_par_type(&mut self) {
        self.pokemons.sort_by(|a, b| {
            let ta = format!("{:?}", a.type_pkm);
            let tb = format!("{:?}", b.type_pkm);
            ta.cmp(&tb)
        });
    }

    fn sauvegarder(&self, fichier: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(fichier)?;

        writeln!(file, "ID,Nom,Niveau,Type,XP,Genre")?;
        for pkm in &self.pokemons {
            writeln!(
                file,
                "{},{},{},{:?},{},{:?}",
                pkm.id, pkm.nom, pkm.niveau, pkm.type_pkm, pkm.experience, pkm.genre
            )?;
        }
        println!("Données enregistrées dans '{}'.", fichier);
        Ok(())
    }

    fn charger(&mut self, fichier: &str) -> io::Result<()> {
        if !Path::new(fichier).exists() {
            println!("Le fichier '{}' est introuvable.", fichier);
            return Ok(());
        }

        if !self.pokemons.is_empty() {
            println!("La nursery n'est pas vide. Videz-là avant de charger un fichier.");
            return Ok(());
        }

        let mut f = File::open(fichier)?;
        let mut contenu = String::new();
        f.read_to_string(&mut contenu)?;

        let mut lignes = contenu.lines();

        // Skip header
        lignes.next();

        for line in lignes {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 6 {
                continue;
            }
            let uuid = Uuid::parse_str(parts[0]).unwrap_or_else(|_| Uuid::new_v4());
            let nom = parts[1].to_string();
            let niv = parts[2].parse::<u32>().unwrap_or(1);

            let type_pkm = match parts[3] {
                "Feu" => PokemonType::Feu,
                "Eau" => PokemonType::Eau,
                "Plante" => PokemonType::Plante,
                "Electrik" => PokemonType::Electrik,
                "Tenebre" => PokemonType::Tenebre,
                _ => PokemonType::Feu,
            };

            let xp = parts[4].parse::<u32>().unwrap_or(0);

            let genre = match parts[5] {
                "Male" => PokemonGender::Male,
                "Femelle" => PokemonGender::Femelle,
                _ => PokemonGender::Male,
            };

            self.pokemons.push(Pokemon {
                id: uuid,
                nom,
                niveau: niv,
                type_pkm,
                experience: xp,
                genre,
            });
        }

        println!("{} Pokémon importés depuis '{}'.", self.pokemons.len(), fichier);
        Ok(())
    }
}

fn afficher_menu() {
    println!("\n~ Système d'Élevage (Nursery) ~");
    println!("1. Ajouter un Pokémon");
    println!("2. Afficher tous les Pokémon");
    println!("3. Entraîner tous les Pokémon");
    println!("4. Reproduction (2 Pokémon)");
    println!("5. Trier par niveau décroissant");
    println!("6. Trier par type alphabétique");
    println!("7. Sauvegarder dans un fichier");
    println!("8. Charger depuis un fichier");
    println!("9. Quitter");
    print!("Votre sélection : ");
    io::stdout().flush().unwrap();
}

fn demander_pokemon() -> Option<Pokemon> {
    println!("Veuillez saisir les informations du Pokémon.");

    print!("Nom (ou 'aléatoire'): ");
    io::stdout().flush().unwrap();
    let mut nom_str = String::new();
    io::stdin().read_line(&mut nom_str).ok()?;
    let nom_str = nom_str.trim().to_string();

    let final_nom = if nom_str.to_lowercase() == "aléatoire" || nom_str.to_lowercase() == "aleatoire" {
        let noms = [
            "Goupix", "Magicarpe", "Germignon", "Caninos",
            "Pikachu", "Miaouss", "Rattata", "Fouinette",
            "Rondoudou", "Evoli", "Psykokwak", "Nosferapti"
        ];
        noms[rand::thread_rng().gen_range(0..noms.len())].to_string()
    } else {
        nom_str
    };

    println!("Type : [1] Feu | [2] Eau | [3] Plante | [4] Electrik | [5] Ténèbre");
    let mut type_str = String::new();
    io::stdin().read_line(&mut type_str).ok()?;
    let type_pkm = match type_str.trim() {
        "1" => PokemonType::Feu,
        "2" => PokemonType::Eau,
        "3" => PokemonType::Plante,
        "4" => PokemonType::Electrik,
        "5" => PokemonType::Tenebre,
        _ => {
            println!("Type inconnu !");
            return None;
        }
    };

    println!("Genre : [1] Mâle | [2] Femelle | [3] Aléatoire");
    let mut genre_str = String::new();
    io::stdin().read_line(&mut genre_str).ok()?;
    let genre_pkm = match genre_str.trim() {
        "1" => PokemonGender::Male,
        "2" => PokemonGender::Femelle,
        "3" => {
            if rand::thread_rng().gen_bool(0.5) {
                PokemonGender::Male
            } else {
                PokemonGender::Femelle
            }
        }
        _ => {
            println!("Genre invalide.");
            return None;
        }
    };

    Some(Pokemon::new(final_nom, type_pkm, genre_pkm))
}

fn choisir_reproduction(nursery: &mut Nursery) {
    if nursery.pokemons.len() < 2 {
        println!("Il faut au moins 2 Pokémon pour une reproduction !");
        return;
    }

    println!("Saisir l'ID du 1er Pokémon :");
    nursery.afficher_tous();
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).ok();
    let first_id = match first_input.trim().parse::<Uuid>() {
        Ok(id_ok) => id_ok,
        Err(_) => {
            println!("ID invalide !");
            return;
        }
    };
    let p1 = match nursery.pokemons.iter().find(|p| p.id == first_id) {
        Some(poke) => poke.clone(),
        None => {
            println!("Aucun Pokémon ne correspond à cet ID.");
            return;
        }
    };

    println!("Saisir l'ID du 2nd Pokémon :");
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).ok();
    let second_id = match second_input.trim().parse::<Uuid>() {
        Ok(id_ok) => id_ok,
        Err(_) => {
            println!("ID invalide !");
            return;
        }
    };
    let p2 = match nursery.pokemons.iter().find(|p| p.id == second_id) {
        Some(poke) => poke.clone(),
        None => {
            println!("Aucun Pokémon ne correspond à cet ID.");
            return;
        }
    };

    println!("Vos Pokémon sélectionnés :");
    p1.afficher();
    p2.afficher();

    nursery.reproduire(&p1, &p2);
}

fn main() {
    let mut nursery = Nursery::new();

    println!("Bienvenue dans la Nursery Pokémon !");
    println!("Vous pouvez gérer un élevage, entraîner et reproduire vos Pokémon.");
    println!("─────────────────────────────────────");
    let mut continuer = true;

    while continuer {
        afficher_menu();
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).ok();

        match choix.trim() {
            "1" => {
                if let Some(poke) = demander_pokemon() {
                    nursery.ajouter_pokemon(poke);
                    println!("Pokémon ajouté !");
                } else {
                    println!("Erreur lors de la saisie !");
                }
            }
            "2" => {
                nursery.afficher_tous();
            }
            "3" => {
                println!("Saisir le nombre d'XP à donner (ex: 50) :");
                let mut xp_str = String::new();
                io::stdin().read_line(&mut xp_str).ok();
                let xp = xp_str.trim().parse::<u32>().unwrap_or(30);
                nursery.entrainer_tous(xp);
                println!("Entraînement terminé !");
                nursery.afficher_tous();
            }
            "4" => {
                choisir_reproduction(&mut nursery);
            }
            "5" => {
                println!("Tri par niveau (descendant)...");
                nursery.tri_par_niveau();
                nursery.afficher_tous();
            }
            "6" => {
                println!("Tri par type (ordre alphabétique)...");
                nursery.tri_par_type();
                nursery.afficher_tous();
            }
            "7" => {
                print!("Nom du fichier où sauvegarder : ");
                io::stdout().flush().unwrap();
                let mut fichier = String::new();
                io::stdin().read_line(&mut fichier).ok();
                let fichier = fichier.trim();
                if let Err(err) = nursery.sauvegarder(fichier) {
                    println!("Erreur de sauvegarde : {}", err);
                }
            }
            "8" => {
                print!("Nom du fichier à charger : ");
                io::stdout().flush().unwrap();
                let mut fichier = String::new();
                io::stdin().read_line(&mut fichier).ok();
                let fichier = fichier.trim();
                if let Err(err) = nursery.charger(fichier) {
                    println!("Erreur de chargement : {}", err);
                }
            }
            "9" => {
                println!("Fermeture du programme. À bientôt !");
                continuer = false;
            }
            _ => println!("Choix invalide, réessayez."),
        }
    }
}
