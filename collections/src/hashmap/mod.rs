use std::collections::HashMap;

pub fn hashmap_practice() {
    new();
    new_iter();
}

fn new() {
    // create mutable empty hash map
    let mut deposits = HashMap::new();
    // hash maps store their data on the heap
    // Like vectors, hash maps are homogeneous:
    // all of the keys must have the same type,
    // and all of the values must have the same type.
    deposits.insert(String::from("기업"), 10_000_000);
    deposits.insert(String::from("NH 투자"), 50_000_000);
}

fn new_iter() {
    let banks = vec![String::from("기업"), String::from("NH 투자")];
    let balances = vec![10_000_000, 50_000_000];

    let deposits: HashMap<_, _> = banks.into_iter().zip(balances.into_iter()).collect();
    // banks and balances are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", deposits);
    
    let temas = vec![String::from("Blue"), String::from("Red")];
    let scores = vec![50, 70];
    let team_scores = temas.into_iter().zip(scores.into_iter()).collect::<HashMap<String, i32>>();
    println!("{:?}", team_scores);
}