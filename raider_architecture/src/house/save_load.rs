use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use crate::house::room::*;

pub fn house_to_file(house: &House, mut file: File) -> std::io::Result<()> {
    // Write house dimensions and facing direction
    writeln!(
        file,
        "House: {:?}/{:?}/{:?}",
        house.width, house.length, house.facing
    )?;

    // Write rooms
    for room in &house.rooms {
        let position = match &room.position {
            Some(p) => format!("{},{}", p.x, p.y),
            None => "None".to_string(),
        };
        writeln!(
            file,
            "Room: {}:{:?}/{:?}/{:?}/{}",
            room.name, room.room_type, room.length, room.width, position
        )?;

        // Write walls for the room
        for wall in &room.walls {
            writeln!(
                file,
                "Wall: ({},{})-({},{})",
                wall.start.x, wall.start.y, wall.end.x, wall.end.y
            )?;
        }

        // Write windows for the room
        for window in &room.windows {
            writeln!(
                file,
                "Window: ({},{})-({},{})",
                window.start.x, window.start.y, window.end.x, window.end.y
            )?;
        }
    }

    // Write doors
    for door in &house.doors {
        let (room1, room2) = &door.connected_rooms;
        let start = match &door.start {
            Some(p) => format!("{},{}", p.x, p.y),
            None => "None".to_string(),
        };
        let end = match &door.end {
            Some(p) => format!("{},{}", p.x, p.y),
            None => "None".to_string(),
        };
        writeln!(
            file,
            "Door: {}-{}:{:?}/{}/{}",
            room1.name, room2.name, door.door_type, start, end
        )?;
    }

    Ok(())
}

fn clean_coordinate(coord: &str) -> String {
    coord
        .trim_matches(|c: char| !c.is_numeric() && c != '.' && c != '-')
        .to_string()
}

pub fn file_to_house(path: &Path) -> Result<House, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut rooms = Vec::new();
    let mut doors = Vec::new();
    let mut width: Option<f32> = None;
    let mut length: Option<f32> = None;
    let mut facing = Facing::North;

    let mut current_room: Option<Room> = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("House:") {
            let parts: Vec<&str> = line[6..].split('/').map(str::trim).collect();
            width = if parts[0].starts_with("Some(") {
                parts[0][5..parts[0].len() - 1].parse().ok()
            } else {
                None
            };
            length = if parts[1].starts_with("Some(") {
                parts[1][5..parts[1].len() - 1].parse().ok()
            } else {
                None
            };
            facing = match parts[2] {
                "North" => Facing::North,
                "South" => Facing::South,
                "East" => Facing::East,
                "West" => Facing::West,
                _ => Facing::North,
            };
        } else if line.starts_with("Room:") {
            if let Some(room) = current_room.take() {
                rooms.push(room);
            }
            let parts: Vec<&str> = line[5..].split(':').map(str::trim).collect();
            let name = parts[0].to_string();
            let room_info: Vec<&str> = parts[1].split('/').map(str::trim).collect();

            let room_type: RoomType = match room_info[0] {
                "LivingRoom" => RoomType::LivingRoom,
                "Kitchen" => RoomType::Kitchen,
                "Bathroom" => RoomType::Bathroom,
                "Hall" => RoomType::Hall,
                "Toilet" => RoomType::Toilet,
                "Bedroom" => RoomType::Bedroom,
                "Corridor" => RoomType::Corridor,
                "DiningRoom" => RoomType::DiningRoom,
                "Empty" => RoomType::Empty,
                _ => RoomType::Empty,
            };

            let length = if room_info[1].starts_with("Some(") {
                room_info[1][5..room_info[1].len() - 1].parse().ok()
            } else {
                None
            };
            let width = if room_info[2].starts_with("Some(") {
                room_info[2][5..room_info[2].len() - 1].parse().ok()
            } else {
                None
            };

            let position = if room_info.len() > 3 && room_info[3] != "None" {
                let pos: Vec<&str> = room_info[3].split(',').collect();
                if pos.len() == 2 {
                    Some(Point {
                        x: pos[0].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid position x coordinate: {}", e),
                            )
                        })?,
                        y: pos[1].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid position y coordinate: {}", e),
                            )
                        })?,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            current_room = Some(Room {
                name: name.clone(),
                room_type,
                length,
                width,
                position,
                connections: Vec::new(),
                walls: Vec::new(),
                windows: Vec::new(),
            });
        } else if line.starts_with("Wall:") {
            if let Some(room) = current_room.as_mut() {
                let parts: Vec<&str> = line[5..].split('-').collect();
                let start_coords: Vec<String> = parts[0][1..]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();
                let end_coords: Vec<String> = parts[1][1..]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();

                if start_coords.len() != 2 || end_coords.len() != 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid wall coordinates format",
                    )
                    .into());
                }

                let start = Point {
                    x: start_coords[0].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid wall start x coordinate: {}", e),
                        )
                    })?,
                    y: start_coords[1].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid wall start y coordinate: {}", e),
                        )
                    })?,
                };
                let end = Point {
                    x: end_coords[0].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid wall end x coordinate: {}", e),
                        )
                    })?,
                    y: end_coords[1].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid wall end y coordinate: {}", e),
                        )
                    })?,
                };

                room.walls.push(Wall {
                    start: start.clone(),
                    end: end.clone(),
                });
            }
        } else if line.starts_with("Window:") {
            if let Some(room) = current_room.as_mut() {
                let parts: Vec<&str> = line[7..].split('-').collect();
                let start_coords: Vec<String> = parts[0][1..]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();
                let end_coords: Vec<String> = parts[1][1..]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();

                if start_coords.len() != 2 || end_coords.len() != 2 {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid window coordinates format",
                    )
                    .into());
                }

                let start = Point {
                    x: start_coords[0].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid window start x coordinate: {}", e),
                        )
                    })?,
                    y: start_coords[1].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid window start y coordinate: {}", e),
                        )
                    })?,
                };
                let end = Point {
                    x: end_coords[0].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid window end x coordinate: {}", e),
                        )
                    })?,
                    y: end_coords[1].parse().map_err(|e| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("Invalid window end y coordinate: {}", e),
                        )
                    })?,
                };

                room.windows.push(Window {
                    start: start.clone(),
                    end: end.clone(),
                });
            }
        } else if line.starts_with("Door:") {
            let parts: Vec<&str> = line[5..].split(':').map(str::trim).collect();
            let room_names: Vec<&str> = parts[0].split('-').map(str::trim).collect();
            let door_info: Vec<&str> = parts[1].split('/').map(str::trim).collect();

            if room_names.len() != 2 || door_info.len() != 3 {
                return Err(
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid door format").into(),
                );
            }

            let room1_name = room_names[0].to_string();
            let room2_name = room_names[1].to_string();

            let door_type: DoorType = if door_info[0] == "Sliding" {
                DoorType::Sliding
            } else if door_info[0] == "Hinged { opens_in: true }" {
                DoorType::Hinged { opens_in: true }
            } else {
                DoorType::Hinged { opens_in: false }
            };

            let start = if door_info[1] != "None" {
                let pos: Vec<String> = door_info[1]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();
                if pos.len() == 2 {
                    Some(Point {
                        x: pos[0].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid door start x coordinate: {}", e),
                            )
                        })?,
                        y: pos[1].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid door start y coordinate: {}", e),
                            )
                        })?,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            let end = if door_info[2] != "None" {
                let pos: Vec<String> = door_info[2]
                    .split(',')
                    .map(|s| clean_coordinate(s))
                    .collect();
                if pos.len() == 2 {
                    Some(Point {
                        x: pos[0].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid door end x coordinate: {}", e),
                            )
                        })?,
                        y: pos[1].parse().map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Invalid door end y coordinate: {}", e),
                            )
                        })?,
                    })
                } else {
                    None
                }
            } else {
                None
            };

            doors.push((
                room1_name.clone(),
                room2_name.clone(),
                Door {
                    door_type,
                    connected_rooms: (
                        Room {
                            name: room1_name.clone(),
                            ..Default::default()
                        },
                        Room {
                            name: room2_name.clone(),
                            ..Default::default()
                        },
                    ),
                    start,
                    end,
                },
            ));
        }
    }

    if let Some(room) = current_room {
        rooms.push(room);
    }

    let mut house = House {
        rooms,
        width,
        length,
        facing,
        doors: vec![],
    };

    for (room1_name, room2_name, mut door) in doors {
        if let (Some(room1), Some(room2)) = (
            house.rooms.iter().find(|r| r.name == room1_name).cloned(),
            house.rooms.iter().find(|r| r.name == room2_name).cloned(),
        ) {
            door.connected_rooms = (room1, room2);
            house.doors.push(door);
        } else {
            return Err(
                io::Error::new(io::ErrorKind::InvalidData, "Invalid room names in door").into(),
            );
        }
    }

    Ok(house)
}
