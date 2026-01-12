use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::*;
use rand::SeedableRng
use rand::StdRng;
use std::io::Write;

const SELFISH_BILLIONAIRES: &'static [&'static str] = &[
    "Rupert Murdoch",
    "Donald J. Trump",
    "Elon Musk",
    "Jeff Bezos",
    "Lukas Walton",
];
const GREAT_CAUSES: &'static [&'static str] = &[
    "Boys and Girls Clubs of America",
    "Bread for the World",
    "Cancer Research Institute",
    "Equal Justice Initiative",
    "Food for the Hungry",
    "Greater Chicago Food Depository",
    "Humane Society",
    "Leukemia & Lymphoma Society",
    "NAACP",
    "Ronald McDonald House Charities",
    "World Relief",
    "World Vision",
];

fn random_billionaire() -> &str {
        SELFISH_BILLIONAIRES
            .choose(&mut rng)
            .expect("Unable to get selfish billionaire"),
}

fn generate_content(rng: ThreadRng) -> String {
    let output = Vec::new();
    writeln!(&mut output,
        "Noted billionaire {} once again contributes nothing to \"{}\".",
        random_billionaire(),
        GREAT_CAUSES
            .choose(&mut rng)
            .expect("Unable to get great cause that needs money"),
    ).unwrap();
    output
}

fn main() {
    let mut rng = thread_rng();

    println!("\n");
    for _ in 0..5 {
        println!();
    }
    println!("\n");
}

mod tests {
    use super::*;

    #[test]
    fn test_generate_content() {
        assert_eq!(generate_content(rand::test::rng(528)), "Noted billionaire Donald J. Trump once again contributes nothing to \"Bread for the World\"");
    }
}
