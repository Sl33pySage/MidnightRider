use std::io;

//1. To create an object in Rust, you define a custom struct and instantiate it by providing values
// for its fields. Unlike Traditional OOP languages, Rust doesn;t use classes; instead, you define
// data in structs and attach behaviors using implementation blocks
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
        name: "Plato's cave",
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

    let _map: [Room; 4] = [STARTER_ROOM, FOREST_ROOM, CITY_ROOM, LIBRARY_ROOM];

    // Start of the game???
    println!("{}", STARTER_ROOM.name);
    print!(
        "Where do you want to go? North:{}\nEast:{}\nSouth:{}\nWest:{}\n",
        STARTER_ROOM.n, STARTER_ROOM.e, STARTER_ROOM.s, STARTER_ROOM.w
    );
    println!("Please enter your name:");

    let mut name: String = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {name}");
}
