use std::io;

//1. To create an object in Rust, you define a custom struct and instantiate it by providing values
// for its fields. Unlike Traditional OOP languages, Rust doesn;t use classes; instead, you define
// data in structs and attach behaviors using implementation blocks
#[derive(Debug)] // Allows for printing with {:?}
struct Room {
    name: &'static str,
    n: i32,
    e: i32,
    s: i32,
    w: i32,
}

fn main() {
    //2. To create an instance of the struct, declare a variable and define the values for each field.
    const STARTER_ROOM: Room = Room {
        name: "Plato's Cave",
        n: 1,
        e: 2,
        s: 3,
        w: 4,
    };

    const FOREST_ROOM: Room = Room {
        name: "Forest",
        n: -1,
        e: 2,
        s: -1,
        w: 1,
    };

    const CITY_ROOM: Room = Room {
        name: "City",
        n: 0,
        e: -1,
        s: 3,
        w: -1,
    };

    const LIBRARY_ROOM: Room = Room {
        name: "Library",
        n: 4,
        e: -1,
        s: -1,
        w: -1,
    };

    /*              GAMEMAP
     *
     *          Start/Plato's Cave -- Forest
     *               |
     *           City/CITY_ROOM ----- Library
     *  */

    let _map = [STARTER_ROOM, FOREST_ROOM, CITY_ROOM, LIBRARY_ROOM];
    let a = &_map[0];
    println!("{:?}", a);
    // Start of the game???

    println!("Please enter your name:");

    let mut player_name: String = String::new();

    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    println!("Are you sure you want to be named: {player_name}");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    println!("I don't think that fits you...");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");

    println!("You shall be named Gobby");
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read line");
    println!("Too bad Gobby...");
    println!("You start in {} Gobby...", STARTER_ROOM.name);
    print!(
        "Where do you want to go?\nNorth:{}\nEast:{}\nSouth:{}\nWest:{}\n",
        STARTER_ROOM.n, STARTER_ROOM.e, STARTER_ROOM.s, STARTER_ROOM.w
    );
}
