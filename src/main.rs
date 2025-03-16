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

    print_location(&locations, &links, &entities, &objects, 1, 1);
}

fn print_location(
    locations: &Vec<Location>,
    links: &Vec<String>,
    entities: &Vec<Entity>,
    objects: &Vec<Object>,
    location_id: usize,
    entity_id: usize,
) {
    print!("u r in {}\n", locations[location_id].name);

    // prit objects
    print!("u c: ");
    let mut counter = 0;
    for &oid in &locations[location_id].objects {
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", objects[oid].name);
    }
    if counter == 0 {
        print!("nothing");
    }
    print!("\n");

    // print entity excluding the current entity
    let mut counter = 0;
    for &eid in locations[location_id].entities.iter() {
        if eid == entity_id {
            continue;
        }
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", entities[eid].name);
    }
    if counter != 0 {
        print!(" is here\n");
    }

    // print exits
    print!("exits: ");
    let mut counter = 0;
    for lid in locations[location_id].links.iter() {
        if counter != 0 {
            print!(", ");
        }
        counter += 1;
        print!("{}", links[lid.link]);
    }
    if counter == 0 {
        print!("none");
    }
    print!("\n");
}
