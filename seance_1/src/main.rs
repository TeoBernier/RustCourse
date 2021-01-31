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
    // à remplir
}

enum VecOption {
    VecSome(usize),
    VecNone,
}

// Il n'est pas nécessaire de modifier le main pour la solution initiale,
// mais si vous avez d'autres idées, elles sont les bienvenues.
// Vous ne devez juste pas utiliser les itérateurs déjà implémentés.
fn main() {
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
