use rand::prelude::IteratorRandom;
use rand::Rng;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq)]
pub struct Door {
    pub door_type: DoorType,
    pub connected_rooms: (Room, Room),
    pub start: Option<Point>,
    pub end: Option<Point>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wall {
    pub start: Point,
    pub end: Point,
}

impl Default for Wall {
    fn default() -> Self {
        Wall {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 0.0, y: 0.0 },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DoorType {
    Sliding,
    Hinged { opens_in: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub name: String,
    pub room_type: RoomType,
    pub width: Option<f32>,
    pub length: Option<f32>,
    pub position: Option<Point>,
    pub connections: Vec<Door>,
    pub walls: Vec<Wall>,
    pub windows: Vec<Window>,
}

impl Room {
    pub fn new(
        name: &str,
        room_type: RoomType,
        width: Option<f32>,
        length: Option<f32>,
        position: Option<Point>,
    ) -> Room {
        Room {
            name: name.to_string(),
            room_type,
            width,
            length,
            position,
            connections: Vec::new(),
            walls: Vec::new(),
            windows: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, door: Door) {
        self.connections.push(door);
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            name: String::new(),
            room_type: RoomType::Empty,
            width: None,
            length: None,
            position: None,
            connections: Vec::new(),
            walls: Vec::new(),
            windows: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RoomType {
    LivingRoom,
    Kitchen,
    Bathroom,
    Hall,
    Toilet,
    Bedroom,
    Corridor,
    DiningRoom,
    Empty,
}

impl RoomType {
    pub fn can_have_multiple_connections(&self) -> bool {
        match self {
            RoomType::Corridor
            | RoomType::LivingRoom
            | RoomType::DiningRoom
            | RoomType::Bedroom
            | RoomType::Hall => true,
            _ => false,
        }
    }

    pub fn allowed_connections(&self) -> Vec<RoomType> {
        match self {
            RoomType::Bedroom => vec![RoomType::Corridor, RoomType::Bathroom, RoomType::LivingRoom],
            RoomType::LivingRoom => vec![
                RoomType::Bedroom,
                RoomType::Kitchen,
                RoomType::Bathroom,
                RoomType::Hall,
                RoomType::Toilet,
                RoomType::Corridor,
                RoomType::DiningRoom,
                RoomType::Empty,
            ],
            RoomType::Kitchen => vec![RoomType::LivingRoom, RoomType::DiningRoom],
            RoomType::Bathroom => vec![
                RoomType::LivingRoom,
                RoomType::Hall,
                RoomType::Bedroom,
                RoomType::Corridor,
            ],
            RoomType::Hall => vec![RoomType::Toilet, RoomType::LivingRoom, RoomType::Corridor],
            RoomType::Toilet => vec![RoomType::Hall, RoomType::LivingRoom, RoomType::Corridor],
            RoomType::Corridor => vec![
                RoomType::LivingRoom,
                RoomType::Bathroom,
                RoomType::Hall,
                RoomType::Toilet,
                RoomType::Bedroom,
                RoomType::DiningRoom,
                RoomType::Empty,
            ],
            RoomType::DiningRoom => vec![
                RoomType::LivingRoom,
                RoomType::Corridor,
                RoomType::Kitchen,
                RoomType::Hall,
            ],
            RoomType::Empty => vec![RoomType::LivingRoom, RoomType::Corridor],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Facing {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub struct House {
    pub rooms: Vec<Room>,
    pub width: Option<f32>,
    pub length: Option<f32>,
    pub facing: Facing,
    pub doors: Vec<Door>,
}

impl House {
    pub fn new(width: Option<f32>, length: Option<f32>, facing: Facing) -> House {
        House {
            rooms: Vec::new(),
            width,
            length,
            facing,
            doors: Vec::new(),
        }
    }

    pub fn connect_rooms(&mut self, room1_name: &str, room2_name: &str, door_type: DoorType) {
        let (room1_index, room2_index) = (
            self.rooms.iter().position(|r| r.name == room1_name),
            self.rooms.iter().position(|r| r.name == room2_name),
        );

        if let (Some(r1_index), Some(r2_index)) = (room1_index, room2_index) {
            let door = Door {
                door_type,
                connected_rooms: (self.rooms[r1_index].clone(), self.rooms[r2_index].clone()),
                start: None,
                end: None,
            };

            self.rooms[r1_index].add_connection(door.clone());
            self.rooms[r2_index].add_connection(door.clone());
            self.doors.push(door);
        }
    }

    pub fn create_rooms(&mut self) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        let house_area = self.width.unwrap() * self.length.unwrap();

        // Ensure existing rooms have width and length
        let mut total_room_area = 0.0;
        if !self.rooms.is_empty() {
            for room in &mut self.rooms {
                if room.width.is_none() {
                    room.width = Some(rng.gen_range(3.0..6.0));
                }
                if room.length.is_none() {
                    room.length = Some(rng.gen_range(3.0..6.0));
                }
                if let (Some(width), Some(length)) = (room.width, room.length) {
                    total_room_area += width * length;
                }
            }
        }

        if total_room_area > house_area {
            return Err("Total area of rooms is greater than the house area".to_string());
        }

        let essential_rooms = vec![
            ("LivingRoom", RoomType::LivingRoom),
            ("Kitchen", RoomType::Kitchen),
            ("Bathroom", RoomType::Bathroom),
            ("Bedroom", RoomType::Bedroom),
            ("Hall", RoomType::Hall),
        ];

        for (name, room_type) in essential_rooms {
            let (width, length) = self.get_random_dimensions(&mut rng);
            total_room_area += width * length;
            if total_room_area > house_area {
                return Err("Total area of rooms is greater than the house area".to_string());
            }
            self.add_room(name, room_type, Some(width), Some(length), None);
        }

        let additional_rooms = vec![
            ("Toilet", RoomType::Toilet),
            ("Corridor", RoomType::Corridor),
            ("DiningRoom", RoomType::DiningRoom),
            ("Empty", RoomType::Empty),
        ];

        for (name, room_type) in additional_rooms.iter().choose_multiple(&mut rng, 3) {
            let (width, length) = self.get_random_dimensions(&mut rng);
            if total_room_area + (width * length) <= house_area {
                total_room_area += width * length;
                self.add_room(name, *room_type, Some(width), Some(length), None);
            }
        }

        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut max_height_in_row = 0.0;

        for room in &mut self.rooms {
            if let (Some(width), Some(length)) = (room.width, room.length) {
                room.position = Some(Point {
                    x: current_x,
                    y: current_y,
                });

                current_x += width;
                if length > max_height_in_row {
                    max_height_in_row = length;
                }

                if current_x > self.width.unwrap_or(100.0) {
                    current_x = 0.0;
                    current_y += max_height_in_row;
                    max_height_in_row = 0.0;
                }
            }
        }

        Ok(())
    }

    fn get_random_dimensions(&self, rng: &mut rand::rngs::ThreadRng) -> (f32, f32) {
        let width = rng.gen_range(3.0..6.0);
        let length = rng.gen_range(3.0..6.0);
        (width, length)
    }

    fn add_room(
        &mut self,
        name: &str,
        room_type: RoomType,
        width: Option<f32>,
        length: Option<f32>,
        position: Option<Point>,
    ) {
        let room = Room {
            name: name.to_string(),
            room_type,
            width,
            length,
            position,
            connections: Vec::new(),
            walls: Vec::new(),
            windows: Vec::new(),
        };
        self.rooms.push(room);
    }

    pub fn create_connections(&mut self) {
        let mut rng = rand::thread_rng();

        let mut fully_connected_rooms = HashSet::new();
        let mut pending_rooms: VecDeque<_> = self.rooms.iter().map(|r| r.name.clone()).collect();

        let start_room = pending_rooms.pop_front().unwrap();
        fully_connected_rooms.insert(start_room.clone());

        while !pending_rooms.is_empty() {
            let current_room = fully_connected_rooms
                .iter()
                .choose(&mut rng)
                .unwrap()
                .clone();
            let current_room_type = self
                .rooms
                .iter()
                .find(|room| room.name == current_room)
                .unwrap()
                .room_type
                .clone();
            let allowed_connections = current_room_type.allowed_connections();

            if let Some(pos) = pending_rooms.iter().position(|r| {
                let room_type = self
                    .rooms
                    .iter()
                    .find(|room| &room.name == r)
                    .unwrap()
                    .room_type
                    .clone();
                allowed_connections.contains(&room_type)
            }) {
                let next_room = pending_rooms.remove(pos).unwrap();
                self.connect_rooms(&current_room, &next_room, DoorType::Sliding);
                fully_connected_rooms.insert(next_room);
            }
        }
    }

    pub fn generate_walls(&mut self) {
        let mut rng = rand::thread_rng();

        for room in &mut self.rooms {
            let width = room.width.unwrap();
            let length = room.length.unwrap();
            let position = room.position.clone().unwrap_or_else(|| Point {
                x: rng.gen_range(0.0..self.width.unwrap_or(20.0)),
                y: rng.gen_range(0.0..self.length.unwrap_or(20.0)),
            });

            room.position = Some(position.clone());

            let top_left = Point {
                x: position.x,
                y: position.y,
            };
            let top_right = Point {
                x: position.x + width,
                y: position.y,
            };
            let bottom_left = Point {
                x: position.x,
                y: position.y + length,
            };
            let bottom_right = Point {
                x: position.x + width,
                y: position.y + length,
            };

            let mut room_walls = vec![
                Wall {
                    start: top_left.clone(),
                    end: top_right.clone(),
                },
                Wall {
                    start: top_right.clone(),
                    end: bottom_right.clone(),
                },
                Wall {
                    start: bottom_right.clone(),
                    end: bottom_left.clone(),
                },
                Wall {
                    start: bottom_left.clone(),
                    end: top_left.clone(),
                },
            ];

            for door in &mut room.connections {
                let (room1, room2) = &door.connected_rooms;
                let room1_position = room1.position.clone().unwrap_or(Point { x: 0.0, y: 0.0 });
                let room2_position = room2.position.clone().unwrap_or(Point { x: 0.0, y: 0.0 });

                let door_position_x = (room1_position.x + room2_position.x) / 2.0;
                let door_position_y = (room1_position.y + room2_position.y) / 2.0;

                door.start = Some(Point {
                    x: door_position_x,
                    y: door_position_y,
                });
                door.end = Some(Point {
                    x: door_position_x + 1.0,
                    y: door_position_y + 1.0,
                });

                if (room1.room_type == RoomType::Kitchen
                    && (room2.room_type == RoomType::LivingRoom
                        || room2.room_type == RoomType::DiningRoom))
                    || (room2.room_type == RoomType::Kitchen
                        && (room1.room_type == RoomType::LivingRoom
                            || room1.room_type == RoomType::DiningRoom))
                {
                    room_walls.retain(|wall| {
                        !(wall.start
                            == Point {
                                x: door_position_x,
                                y: door_position_y,
                            }
                            || wall.end
                                == Point {
                                    x: door_position_x,
                                    y: door_position_y,
                                })
                    });
                }

                // Update house doors
                if let Some(house_door) = self.doors.iter_mut().find(|d| {
                    (d.connected_rooms.0.name == room1.name
                        && d.connected_rooms.1.name == room2.name)
                        || (d.connected_rooms.1.name == room1.name
                            && d.connected_rooms.0.name == room2.name)
                }) {
                    house_door.start = door.start.clone();
                    house_door.end = door.end.clone();
                }
            }

            let window_size = 1.0;
            match self.facing {
                Facing::North => {
                    room.windows.push(Window {
                        start: top_left.clone(),
                        end: Point {
                            x: top_left.x + window_size,
                            y: top_left.y,
                        },
                    });
                    room.windows.push(Window {
                        start: Point {
                            x: top_right.x - window_size,
                            y: top_right.y,
                        },
                        end: top_right.clone(),
                    });
                }
                Facing::South => {
                    room.windows.push(Window {
                        start: bottom_left.clone(),
                        end: Point {
                            x: bottom_left.x + window_size,
                            y: bottom_left.y,
                        },
                    });
                    room.windows.push(Window {
                        start: Point {
                            x: bottom_right.x - window_size,
                            y: bottom_right.y,
                        },
                        end: bottom_right.clone(),
                    });
                }
                Facing::East => {
                    room.windows.push(Window {
                        start: top_right.clone(),
                        end: Point {
                            x: top_right.x,
                            y: top_right.y + window_size,
                        },
                    });
                    room.windows.push(Window {
                        start: Point {
                            x: bottom_right.x,
                            y: bottom_right.y - window_size,
                        },
                        end: bottom_right.clone(),
                    });
                }
                Facing::West => {
                    room.windows.push(Window {
                        start: top_left.clone(),
                        end: Point {
                            x: top_left.x,
                            y: top_left.y + window_size,
                        },
                    });
                    room.windows.push(Window {
                        start: Point {
                            x: bottom_left.x,
                            y: bottom_left.y - window_size,
                        },
                        end: bottom_left.clone(),
                    });
                }
            }

            room.walls = room_walls;
        }
    }

    pub fn display(&self) {
        let mut displayed_connections = HashSet::new();

        for room in &self.rooms {
            println!(
                "Room: {} ({:?}) ({} x {})",
                room.name,
                room.room_type,
                room.width.unwrap_or(0.0),
                room.length.unwrap_or(0.0)
            );
            for door in &room.connections {
                let (room1, room2) = &door.connected_rooms;
                if !displayed_connections.contains(&(room2.name.clone(), room1.name.clone())) {
                    let (other_room, opens_in) = if room1.name == room.name {
                        (room2, true)
                    } else {
                        (room1, false)
                    };
                    match &door.door_type {
                        DoorType::Sliding => {
                            println!(
                                "  Connected to {} with a sliding door at ({}, {}) to ({}, {})",
                                other_room.name,
                                door.start.as_ref().unwrap().x,
                                door.start.as_ref().unwrap().y,
                                door.end.as_ref().unwrap().x,
                                door.end.as_ref().unwrap().y
                            );
                        }
                        DoorType::Hinged { .. } => println!(
                            "  Connected to {} with a hinged door opening {} at ({}, {}) to ({}, {})",
                            other_room.name,
                            if opens_in { "inward" } else { "outward" },
                            door.start.as_ref().unwrap().x,
                            door.start.as_ref().unwrap().y,
                            door.end.as_ref().unwrap().x,
                            door.end.as_ref().unwrap().y
                        ),
                    }
                    displayed_connections.insert((room1.name.clone(), room2.name.clone()));
                }
            }
            println!();

            // Display walls
            for wall in &room.walls {
                println!(
                    "  Wall from ({}, {}) to ({}, {})",
                    wall.start.x, wall.start.y, wall.end.x, wall.end.y
                );
            }

            // Display windows
            for window in &room.windows {
                println!(
                    "  Window from ({}, {}) to ({}, {})",
                    window.start.x, window.start.y, window.end.x, window.end.y
                );
            }
        }
    }
}
