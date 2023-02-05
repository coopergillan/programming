use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let selfish_billionaires = vec![
        "Rupert Murdoch",
        "Donald J. Trump",
        "Elon Musk",
        "Jeff Bezos",
        "Lukas Walton",
    ];

    let great_causes = vec![
        "Humane Society",
        "Boys and Girls Clubs of America",
        "Bread for the World",
        "Cancer Research Institute",
        "Equal Justice Initiative",
        "Greater Chicago Food Depository",
        "Leukemia & Lymphoma Society",
        "Ronald McDonald House Charities",
        "NAACP",
        "World Vision",
        "World Relief",
        "Food for the Hungry",
        "Bread for the World",
    ];

    let mut rng = thread_rng();

    println!("\n");
    for _ in 0..5 {
        println!(
            "Noted billionaire {} once again contributes nothing to \"{}\".",
            selfish_billionaires
                .choose(&mut rng)
                .expect("Unable to get selfish billionaire"),
            great_causes
                .choose(&mut rng)
                .expect("Unable to get great cause that needs money"),
        );
    }
    println!("\n");
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(false, "Could not assert");
    }
}
