use raider_architecture::house::room::*;    
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;


fn house_to_file(house: &House, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    for room in &house.rooms {
        let description = format!("{}: {}/{}", room.roomType, room.length, room.width);

        file.write_all(description.as_bytes())?;

        file.write_all(b"\n")?;
    }

    Ok(())
}

fn file_to_house(path: &Path) -> Result<House, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rooms = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').map(str::trim).collect();

        if parts.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid file format",
            ).into());
        }

        let room_info: Vec<&str> = parts[1].split('/').map(str::trim).collect();
        
        if room_info.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid file format",
            ).into());
        }

        let room_type = parts[0].to_string();
        let length = room_info[0].parse()?;
        let width = room_info[1].parse()?;

        rooms.push(Room {
            roomType: room_type,
            length: length,
            width: width,
        });
    }

    Ok(House { rooms })
}


fn length_house(n: usize) -> usize {
    let mut l = 1;
    for i in 1..n*n {
        if n > i * i {
            l = i+1;
        }
    }
    l
}

fn plan(n: usize) -> Vec<String> {
    let mut house_plan : Vec<String> = vec![];
    let length = length_house(n) * 2 + 1;
    let mut width = 5;
    for i in 1..(n+1) {
        if length_house(i) * ((width - 1)/4) < i {
            width += 4;
        }
    }
    let mut occ_rooms = 0;
    for i in 0..length {
        let mut row = String::with_capacity(width);
        if i % 2 == 1 {
            row.push('|');
            for j in 2..width {
                if (j - 1) % 4 == 0 && occ_rooms < n {
                    row.push('|');
                    occ_rooms += 1;
                }
                else {
                    row.push(' ');
                }
            }
            row.push('|');
        } else {
            row.push('|');
            occ_rooms += 1;
            for j in 2..width {
                if (j - 1) % 4 == 0 {
                    if i == 0 {
                        row.push('|');
                    } else if house_plan[i - 1].chars().nth(j - 1) == Some('|') {
                        row.push('|');
                    } else {
                        row.push('-')
                    }
                }
                else {
                    row.push('-');
                }
            }
            row.push('|');
        }
        house_plan.push(row);
    }
    house_plan  
}

fn test_display(n: usize) {
    let house = plan(n);
    for row in house {
        println!("{}", row);
    }
}

fn test_plan() {
    let living_room = Room {
        roomType: String::from("Living Room"),
        length: 20,
        width: 15,
    };
    let kitchen = Room {
        roomType: String::from("Kitchen"),
        length: 15,
        width: 10,
    };
    let bathroom = Room {
        roomType: String::from("Bathroom"),
        length: 10,
        width: 8,
    };

    let my_house = House {
        rooms: vec![living_room, kitchen, bathroom],
    };

    if let Err(e) = house_to_file(&my_house, "house.txt") {
        eprintln!("Error: {}", e);
    } else {
        println!("Done");
    }


    let path = Path::new("house.txt");
    match file_to_house(&path) {
        Ok(house) => {
            println!("House read from file: {:?}", house);
        }
        Err(e) => {
            eprintln!("Error reading house from file: {}", e);
        }
    }
}


fn main() {
    loop {
        println!("Choisissez une fonction :");
        println!("'1' pour Algo de sauvegarde");
        println!("'2' pour Algo de display");
        println!("Tapez 'q', pour quitter");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Erreur lors de la lecture de l'entrée");

        let choice = choice.trim();
        if choice == "q" {
            break;
        }
        let choice: u32 = match choice.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un nombre valide ou 'q' pour quitter");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Pour tester la beta, vous pouvez modifier et ajouter des stuctures dans la fonction 'test_plan'.");
                test_plan();
                break;
            }
            2 => {
                println!("Combien de pieces ? ");
                let mut c = String::new();
                io::stdin()
                    .read_line(&mut c)
                    .expect("Erreur lors de la lecture de l'entrée");
                let i: usize = c.trim().parse().expect("Veuillez entrer un nombre");
                test_display(i);
                break;
            }
            _ => println!("Choix invalide"),
        }
    }

    // Don't work
    //test_axel();
}