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

    loop {
        print_location(&state, eid, state.entities[eid].location);
        print!("{} > ", state.entities[eid].name);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut words = input.split_whitespace();

        match words.next() {
            Some("n") => action_go(&mut state, eid, 1),
            Some("e") => action_go(&mut state, eid, 2),
            Some("s") => action_go(&mut state, eid, 3),
            Some("w") => action_go(&mut state, eid, 4),
            Some("i") => action_inventory(&state, eid),
            Some("t") => {
                let obj = words.next();
                if obj.is_none() {
                    print!("take what\n\n");
                    continue;
                }
                action_take(&mut state, eid, obj.unwrap());
            }
            Some("d") => {
                let obj = words.next();
                if obj.is_none() {
                    print!("drop what\n\n");
                    continue;
                }
                action_drop(&mut state, eid, obj.unwrap());
            }
            Some("g") => {
                let obj = words.next();
                if obj.is_none() {
                    print!("give what\n\n");
                    continue;
                }
                let to_entity = words.next();
                if to_entity.is_none() {
                    print!("give to whom\n\n");
                    continue;
                }
                action_give(&mut state, eid, obj.unwrap(), to_entity.unwrap());
            }
            Some("help") => print_help(),
            Some("q") => break,
            None | _ => print!("not understood\n\n"),
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

    // print entities excluding the current entity
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
    let link = match state.locations[source_location_id]
        .links
        .iter()
        .find(|&loc_lnk| loc_lnk.link == link_id)
    {
        Some(loc_lnk) => loc_lnk,
        None => {
            println!("cannot go there\n\n");
            return;
        }
    };

    let destination_location_id = link.location;

    state.locations[destination_location_id]
        .entities
        .push(entity_id);

    state.locations[source_location_id]
        .entities
        .retain(|&x| x != entity_id);

    entity.location = destination_location_id;
}

fn action_take(state: &mut State, entity_id: usize, object_name: &str) {
    let entity = &mut state.entities[entity_id];
    let location = &mut state.locations[entity.location];

    let object_id = match location
        .objects
        .iter()
        .find(|&&oid| state.objects[oid].name == object_name)
    {
        Some(&oid) => oid,
        None => {
            println!("{} not here\n\n", object_name);
            return;
        }
    };

    location.objects.retain(|&x| x != object_id);
    entity.objects.push(object_id);
}

fn action_inventory(state: &State, entity_id: usize) {
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

    let object_id = match entity
        .objects
        .iter()
        .find(|&&oid| state.objects[oid].name == object_name)
    {
        Some(&oid) => oid,
        None => {
            println!("u don't have {}\n\n", object_name);
            return;
        }
    };

    entity.objects.retain(|&x| x != object_id);
    location.objects.push(object_id);
}

fn action_give(state: &mut State, entity_id: usize, object_name: &str, to_entity: &str) {
    let location = &mut state.locations[state.entities[entity_id].location];

    let target_id = match location
        .entities
        .iter()
        .find(|&&eid| state.entities[eid].name == to_entity)
    {
        Some(&eid) => eid,
        None => {
            println!("{} not here\n\n", to_entity);
            return;
        }
    };

    let object_id = match state.entities[entity_id]
        .objects
        .iter()
        .find(|&&oid| state.objects[oid].name == object_name)
    {
        Some(&oid) => oid,
        None => {
            println!("{} not in inventory\n\n", object_name);
            return;
        }
    };

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
