#[macro_use]

extern crate clap;
extern crate rand;

use clap::App;
use rand::Rng;

fn main() {
    let yaml = load_yaml!("options-en.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let dog = matches.value_of("NAME")
        .expect("Parse failed unexpectedly. Please check input.");
    let doggo_compliment = rand::thread_rng().gen_range(0, 10);
    let male: bool = matches.occurrences_of("male") == 1 ;
    let mut s1 = String::new();
    let mut s2 = String::new();
    if male {
        s1 = "boy".to_string();
        s2 = "his".to_string();
    }
    else {
        s1 = "girl".to_string();
        s2 = "her".to_string();
    }
    match doggo_compliment {
        0 => println!("{} is a good doggo.", dog),
        1 => println!("{} is a good woofer.", dog),
        2 => println!("{} is a good {}!", dog, s1),
        3 => println!("Wow, {} - what a decent canine.",dog),
        4 => println!("They're good dogs, Brent."),
        5 => println!("{} is a good h*ckin doggo.", dog),
        6 => println!("{} hasn't chewed on toilet paper since {} last birthday. Wow.", dog, s2),
        7 => println!("11/10"),
        8 => println!("{} is a d*ng fine woofer.", dog),
        9 => println!("H*ckin' heck wow {} is such a good doggo.", dog),
        _ => println!("Tremendous pupper by all conceivable metrics")
    }
}
