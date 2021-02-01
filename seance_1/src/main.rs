#![allow(unused_imports, dead_code, unused_variables)]
/**
 * Le but est de faire un itérateur
 * sur les données du Vecteur.
 *
 * A noter que l'on peut utiliser v[i] pour avoir le (i + 1)ème élément
 * et v.len() pour avoir la longueur du vecteur.
 **/

pub struct IterArray {
    v: Vec<usize>,
    // à remplir
}

impl IterArray {
    // Créer la fonction new (le code ressemblera pas mal à ce qu'il y a dans lib.rs)

    // Remplir la fonction suivant
    //
    // Ca n'est pas important de le comprendre tout de suite :
    //      Le '&mut self' sert ici, comme pour la fonction 'set_age' de Person
    //      à changer l'objet, sans donner notre iterateur.
    pub fn suivant(&mut self) -> VecOption {
        // à remplir
    }
}

pub enum VecOption {
    VecSome(usize),
    VecNone,
}

// Il n'est pas nécessaire de modifier le main pour la solution initiale,
// mais si vous avez d'autres idées, elles sont les bienvenues.
// Vous ne devez juste pas utiliser les itérateurs déjà implémentés nativement dans Rust.
fn main() {
    use seance_1::use_person;
    use_person();

    use VecOption::*;
    let v = vec![1, 2, 3, 4, 5];

    let mut iter = IterArray::new(v);

    // ici, on donne v à l'itérateur
    // faire un print ne compilerait pas
    // car v ne nous appartient plus.
    // println!("{:?}", v);

    print!("[");
    while let VecSome(number) = iter.suivant() {
        print!("{}, ", number);
    }
    println!("]");
}
