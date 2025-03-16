use std::io::Write;

struct Object {
    name: String,
}

struct Entity {
    name: String,
    location: usize,
    objects: Vec<usize>,
}

struct LocationLink {
    link: usize,
    location: usize,
}

struct Location {
    name: String,
    links: Vec<LocationLink>,
    objects: Vec<usize>,
    entities: Vec<usize>,
}

struct State {
    locations: Vec<Location>,
    links: Vec<String>,
    entities: Vec<Entity>,
    objects: Vec<Object>,
}

fn main() {
    let objects: Vec<Object> = vec![
        Object {
            name: String::from(""),
        },
        Object {
            name: String::from("notebook"),
        },
        Object {
            name: String::from("mirror"),
        },
        Object {
            name: String::from("lighter"),
        },
    ];

    let entities: Vec<Entity> = vec![
        Entity {
            name: String::from(""),
            location: 0,
            objects: vec![],
        },
        Entity {
            name: String::from("me"),
            location: 1,
            objects: vec![2],
        },
        Entity {
            name: String::from("u"),
            location: 2,
            objects: vec![],
        },
    ];

    let links: Vec<String> = vec![
        String::from(""),
        String::from("north"),
        String::from("east"),
        String::from("south"),
        String::from("west"),
    ];

    let locations: Vec<Location> = {
        vec![
            Location {
                name: String::from(""),
                links: vec![],
                objects: vec![],
                entities: vec![],
            },
            Location {
                name: String::from("roome"),
                links: vec![
                    LocationLink {
                        link: 1,
                        location: 2,
                    },
                    LocationLink {
                        link: 2,
                        location: 3,
                    },
                    LocationLink {
                        link: 4,
                        location: 4,
                    },
                ],
                objects: vec![],
                entities: vec![1],
            },
            Location {
                name: String::from("office"),
                links: vec![LocationLink {
                    link: 3,
                    location: 1,
                }],
                objects: vec![1, 3],
                entities: vec![2],
            },
            Location {
                name: String::from("bathroom"),
                links: vec![],
                objects: vec![],
                entities: vec![],
            },
            Location {
                name: String::from("kitchen"),
                links: vec![LocationLink {
                    link: 2,
                    location: 1,
                }],
                objects: vec![],
                entities: vec![],
            },
        ]
    };

    let mut state = State {
        locations,
        links,
        entities,
        objects,
    };

    let mut eid = 1;

    // infinite loop
    loop {
        print_location(&state, eid, state.entities[eid].location);

        // print  entity name and prompt
        print!("{} > ", state.entities[eid].name);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // extract first word from input and put the rest in args
        let mut input = input.split_whitespace();
        let cmd = input.next();
        if cmd.is_none() {
            print!("not understood\n\n");
            continue;
        }
        let cmd = cmd.unwrap();

        if cmd == "q" {
            break;
        }

        if cmd == "help" {
            print_help();
        } else if cmd == "n" {
            action_go(&mut state, eid, 1);
        } else if cmd == "e" {
            action_go(&mut state, eid, 2);
        } else if cmd == "s" {
            action_go(&mut state, eid, 3);
        } else if cmd == "w" {
            action_go(&mut state, eid, 4);
        } else if cmd == "i" {
            action_inventory(&mut state, eid);
        } else if cmd == "t" {
            let obj = input.next();
            if obj.is_none() {
                print!("take what\n\n");
                continue;
            }
            action_take(&mut state, eid, obj.unwrap());
        } else if cmd == "d" {
            let obj = input.next();
            if obj.is_none() {
                print!("drop what\n\n");
                continue;
            }
            action_drop(&mut state, eid, obj.unwrap());
        } else if cmd == "g" {
            let obj = input.next();
            if obj.is_none() {
                print!("give what\n\n");
                continue;
            }

            let to_entity = input.next();
            if to_entity.is_none() {
                print!("give to whom\n\n");
                continue;
            }

            action_give(&mut state, eid, obj.unwrap(), to_entity.unwrap());
        } else {
            print!("not understood\n\n");
        }

        if eid == 1 {
            eid = 2;
        } else {
            eid = 1;
        }
    }
}

fn print_location(state: &State, entity_id: usize, location_id: usize) {
    print!("u r in {}\n", state.locations[location_id].name);

    // prit objects
    print!("u c: ");
    let mut counter = 0;
    for &oid in &state.locations[location_id].objects {
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", state.objects[oid].name);
    }
    if counter == 0 {
        print!("nothing");
    }
    print!("\n");

    // print entity excluding the current entity
    let mut counter = 0;
    for &eid in state.locations[location_id].entities.iter() {
        if eid == entity_id {
            continue;
        }
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", state.entities[eid].name);
    }
    if counter != 0 {
        print!(" is here\n");
    }

    // print exits
    print!("exits: ");
    let mut counter = 0;
    for lid in state.locations[location_id].links.iter() {
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", state.links[lid.link]);
    }
    if counter == 0 {
        print!("none");
    }
    print!("\n");
}

fn action_go(state: &mut State, entity_id: usize, link_id: usize) {
    let entity = &mut state.entities[entity_id];
    let source_location_id = entity.location;

    // find the link based on the link_id
    let location_link = state.locations[source_location_id]
        .links
        .iter()
        .find(|&id| id.link == link_id);

    if location_link.is_none() {
        println!("cannot go there\n\n");
        return;
    }

    let lnk = location_link.unwrap();

    if lnk.location == entity.location {
        return;
    }

    let destination_location_id = lnk.location;

    state.locations[destination_location_id]
        .entities
        .push(entity_id);

    state.locations[source_location_id]
        .entities
        .retain(|&id| id != entity_id);

    entity.location = destination_location_id;
}

fn action_take(state: &mut State, entity_id: usize, object_name: &str) {
    let entity = &mut state.entities[entity_id];
    let location = &mut state.locations[entity.location];

    let object_id = location
        .objects
        .iter()
        .find(|&id| state.objects[*id].name == object_name);

    if object_id.is_none() {
        println!("{} not here\n\n", object_name);
        return;
    }

    let object_id = *object_id.unwrap();

    location.objects.retain(|&x| x != object_id);
    entity.objects.push(object_id);
}

fn action_inventory(state: &mut State, entity_id: usize) {
    let entity = &state.entities[entity_id];

    print!("u have: ");
    let mut counter = 0;
    for &oid in &entity.objects {
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", state.objects[oid].name);
    }
    if counter == 0 {
        print!("nothing");
    }
    print!("\n");
}

fn action_drop(state: &mut State, entity_id: usize, object_name: &str) {
    let entity = &mut state.entities[entity_id];
    let location = &mut state.locations[entity.location];

    let object_id = entity
        .objects
        .iter()
        .find(|&id| state.objects[*id].name == object_name);

    if object_id.is_none() {
        println!("u don't have {}\n\n", object_name);
        return;
    }

    let object_id = *object_id.unwrap();

    entity.objects.retain(|&x| x != object_id);
    location.objects.push(object_id);
}

fn action_give(state: &mut State, entity_id: usize, object_name: &str, to_entity: &str) {
    let location = &mut state.locations[state.entities[entity_id].location];

    let target_id = location
        .entities
        .iter()
        .find(|&id| state.entities[*id].name == to_entity);

    if target_id.is_none() {
        println!("{} not here\n\n", to_entity);
        return;
    }

    let target_id = *target_id.unwrap();

    let object_id = state.entities[entity_id]
        .objects
        .iter()
        .find(|&id| state.objects[*id].name == object_name);

    if object_id.is_none() {
        println!("{} not in inventory\n\n", object_name);
        return;
    }

    let object_id = *object_id.unwrap();

    state.entities[entity_id]
        .objects
        .retain(|&x| x != object_id);
    state.entities[target_id].objects.push(object_id);
}

fn print_help() {
    println!(
        "\ncommand:\n  n: go north\n  e: go east\n  s: go south\n  w: go west\n  i: display inventory\n  t <object>: take object\n  d <object>: drop object\n  g <object> <entity>: give object to entity\n  sdr <sector>: read sector from SD card\n  sdw <sector> <text>: write sector to SD card\n  help: this message\n"
    );
}
