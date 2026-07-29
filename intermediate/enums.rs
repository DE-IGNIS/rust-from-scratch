// Using an enum just for the sake of a clean `match`
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn main() {
    let today = Day::Wednesday;

    match today {
        Day::Monday => println!("Back to work!"),
        Day::Tuesday | Day::Wednesday | Day::Thursday => {
            println!("Mid‑week vibes.");
        }
        Day::Friday => println!("Weekend is near!"),
        Day::Saturday | Day::Sunday => println!("Enjoy the weekend!"),
    }
}
