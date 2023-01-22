use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Area {
    Kitchen,
    Bathroom,
    LivingRoom,
    Entrance,
    Outdoor,
    Everywhere,
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Group {
    Bathroom,
    Trashs,
    WipeKitchen,
    Vacuum,
    Outdoor,
    Other,
    Default,
}

pub fn get_groupped_task() -> HashMap<i32, Vec<(Area, &'static str, Group)>> {
    let mut m = HashMap::new();
    m.insert(
        1,
        vec![
            (Area::Kitchen, "Descale and clean Kettle", Group::Other),
            (Area::Kitchen, "Clean Toaster", Group::Other),
            (Area::Kitchen, "Clean Sink", Group::Other),
            (Area::Kitchen, "Clean micro", Group::Other),
            (
                Area::Kitchen,
                "Clean the 3 vases (cloths and dish brush + onion + pot spoon, palette knife ect)",
                Group::Other,
            ),
            (
                Area::LivingRoom,
                "Wipe all surfaces Living room (incl. panels)",
                Group::Other,
            ),
            (
                Area::Kitchen,
                "Wash towels + Cloths (90 degrees)",
                Group::Other,
            ),
            (
                Area::Everywhere,
                "Water plants in common areas",
                Group::Other,
            ),
        ],
    );
    m.insert(
        2,
        vec![
            (Area::Kitchen, "Clean Oven + Trays ", Group::Other),
            (
                Area::LivingRoom,
                "Clean shoe rack, wipe surfaces hall way (incl. panels)",
                Group::Other,
            ),
            (
                Area::Kitchen,
                "Clean Common shelves in fridge",
                Group::Other,
            ),
        ],
    );
    m.insert(
        3,
        vec![(Area::Kitchen, "Wipe kitchen tasks", Group::WipeKitchen)],
    );
    m.insert(5, vec![(Area::Outdoor, "Outdoor tasks", Group::Outdoor)]);
    m.insert(
        8,
        vec![(Area::Kitchen, "Trash related tasks", Group::Trashs)],
    );
    m.insert(
        9,
        vec![
            (Area::Bathroom, "Clean Bathroom tasks", Group::Bathroom),
            (Area::LivingRoom, "Vacuuming tasks", Group::Vacuum),
        ],
    );
    m
}

pub fn subtasks_per_groupped_tasks() -> HashMap<Group, Vec<(Area, &'static str, Group)>> {
    let mut m = HashMap::new();
    m.insert(
        Group::Bathroom,
        vec![
            (Area::Bathroom, "Clean mirror", Group::Default),
            (Area::Bathroom, "Clean sink + tap", Group::Default),
            (
                Area::Bathroom,
                "Clean shower (Floor - Shower head)",
                Group::Default,
            ),
            (Area::Bathroom, "Wipe all surfaces", Group::Default),
            (Area::Bathroom, "Clean toilet", Group::Default),
        ],
    );
    m.insert(
        Group::Trashs,
        vec![
            (Area::Bathroom, "Empty trash bin", Group::Default),
            (
                Area::Kitchen,
                "Empty Trash + Bio Trash + clean bio bin",
                Group::Default,
            ),
            (
                Area::Kitchen,
                "Empty Recycling + clean bins",
                Group::Default,
            ),
            (Area::Kitchen, "Clean under sink", Group::Default),
        ],
    );
    m.insert(
        Group::Vacuum,
        vec![
            (Area::Bathroom, "Vacuum floor + wash", Group::Default),
            (Area::Kitchen, "Vacuum floor + wash", Group::Default),
            (Area::LivingRoom, "Vacuum sofa and chair", Group::Default),
            (Area::LivingRoom, "Vacuum floor + wash", Group::Default),
        ],
    );
    m.insert(
        Group::Outdoor,
        vec![
            (Area::Outdoor, "Refund bottles and cans", Group::Default),
            (
                Area::Outdoor,
                "Shopping (have a look + shoppinglist)",
                Group::Default,
            ),
        ],
    );
    m.insert(
        Group::WipeKitchen,
        vec![
            (
                Area::Kitchen,
                "Kitchen counter area: Wipe all surfaces + panels",
                Group::Default,
            ),
            (
                Area::Kitchen,
                "Table area: Wipe all surfaces + panels",
                Group::Default,
            ),
        ],
    );
    m
}
