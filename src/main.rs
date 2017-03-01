#[macro_use]

extern crate clap;
extern crate rand;
extern crate chrono;

use clap::App;
use rand::Rng;
use chrono::Local;

fn main() {
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let dog = matches.value_of("NAME")
        .expect("Parse failed unexpectedly. Please check input.");
    let doggo_compliment = rand::thread_rng().gen_range(0, 23);
    let male = matches.occurrences_of("male") == 1;
    let (s1, s2, s3) =
        if male {
            ("boy", "his", "him")
        }
        else {
            ("girl", "her", "her")
        };
    let date = Local::now();
    match doggo_compliment {
        0 => println!("{} is a good doggo.", dog),
        1 => println!("{} is a good woofer.", dog),
        2 => println!("{} is a good {}!", dog, s1),
        3 => println!("Wow, {} - what a decent canine.",dog),
        4 => println!("{}? D*ng that is a good dog.",dog),
        5 => println!("{} wags {} tail so h*ckdarn well.",dog,s2),
        6 => println!("I came to bury bones, not to praise {}. Just kidding {} is floofin' fine as h*ck.",dog, dog),
        7 => println!("They're good dogs, Brent."),
        8 => println!("Holy h*ck I've never seen such a solid woofer."),
        9 => println!("{} is a good h*ckin doggo.", dog),
        10 => println!("{} hasn't chewed on toilet paper since {} last birthday. Wow.", dog, s2),
        11 => println!("11/10 excellent bork & tail wagging"),
        12 => println!("{} is a d*ng fine woofer.", dog),
        13 => println!("H*ckin' heck wow {} is such a good doggo.", dog),
        14 => println!("'bork bork' - {}, {}", dog, date.format("%Y")),
        15 => println!("Tremendous pupper by all conceivable metrics"),
        16 => println!("I'm having a lot of feelings about how great {} is at being a pupper.", dog),
        17 => println!("17/10 {} is in {} prime", dog, s2),
        18 => println!("11.5/10 {} is a very effective floof", dog),
        19 => println!("Too much floof, 12/10 would cuddle with {} all day long.", dog),
        20 => println!("Very effective fetcher 5 stars holy h*ck"),
        21 => println!("{} only pooped on the bed once this week and that's really good for {}. Nice.", dog, s3),
        22 => println!("{} is technically a cat but don't tell {} it will hurt {} feelings.", dog, s3, s2),
        _ => println!("IT'S REALLY HARD TO OVERSTATE HOW MUCH I LIKE THIS WOOFMACHINE")
    }
}
