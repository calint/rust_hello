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
    let mut objects: Vec<Object> = vec![
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

    let mut entities: Vec<Entity> = vec![
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

    let mut locations: Vec<Location> = {
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

    let eid = 1;

    print_location(&state, eid, state.entities[eid].location);
    action_go(&mut state, eid, 1);
    print_location(&state, eid, state.entities[eid].location);
    action_go(&mut state, eid, 3);
    print_location(&state, eid, state.entities[eid].location);
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
        .find(|&x| x.link == link_id);

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
        .retain(|&x| x != entity_id);

    entity.location = destination_location_id;
}
