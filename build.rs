use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Formatter, Pointer};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::Split;

trait FromValueSplit {
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self;
}

enum ObjectInformationExtra {
    None,
    Treasure(Vec<u16>),
}

impl Debug for ObjectInformationExtra {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectInformationExtra::None => write!(f, "ObjectInformationExtra::None"),
            ObjectInformationExtra::Treasure(treasure) => {
                write!(f, "ObjectInformationExtra::Treasure(&{:?})", treasure)
            }
        }
    }
}

#[derive(Debug)]
struct ObjectInformation {
    pub name: String,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: String,
    pub display_name: String,
    pub description: String,
    pub extra: ObjectInformationExtra,
}

impl FromValueSplit for ObjectInformation {
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self {
        let extra: ObjectInformationExtra = match id {
            535u16 | 536u16 | 537u16 | 749u16 | 275u16 => ObjectInformationExtra::Treasure(
                value_split[6usize]
                    .split(" ")
                    .map(|treasure: &str| treasure.parse::<u16>().unwrap())
                    .collect(),
            ),
            _ => ObjectInformationExtra::None,
        };

        Self {
            name: value_split[0usize].to_string(),
            price: value_split[1usize].parse::<u32>().unwrap(),
            edibility: value_split[2usize].parse::<i16>().unwrap(),
            type_and_category: value_split[3usize].to_string(),
            display_name: value_split[4usize].to_string(),
            description: value_split[5usize].to_string(),
            extra,
        }
    }
}

#[derive(Debug)]
struct BigCraftablesInformation {
    pub name: String,
    pub price: u32,
    pub edibility: i16,
    pub type_and_category: String,
    pub description: String,
    pub can_be_set_outdoors: bool,
    pub can_be_set_indoors: bool,
    pub fragility: u8,
    pub display_name: String,
}

impl FromValueSplit for BigCraftablesInformation {
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self {
        Self {
            name: value_split[0usize].to_string(),
            price: value_split[1usize].parse::<u32>().unwrap(),
            edibility: value_split[2usize].parse::<i16>().unwrap(),
            type_and_category: value_split[3usize].to_string(),
            description: value_split[4usize].to_string(),
            can_be_set_outdoors: value_split[5usize].parse::<bool>().unwrap(),
            can_be_set_indoors: value_split[6usize].parse::<bool>().unwrap(),
            fragility: value_split[7usize].parse::<u8>().unwrap(),
            display_name: value_split[8usize].to_string(),
        }
    }
}

#[derive(Debug)]
struct Furniture {
    pub name: String,
    pub type_: String,
    pub source_rectangle_width: u8,
    pub source_rectangle_height: u8,
    pub bounding_box_width: u8,
    pub bounding_box_height: u8,
    pub rotations: u8,
    pub price: u32,
}

impl FromValueSplit for Furniture {
    // Integer types (Furniture.getTypeNumberFromName):
    // 0 = chair
    // 1 = bench
    // 2 = couch
    // 3 = armchair
    // 4 = dresser
    // 5 = long table
    // 6 = painting
    // 7 = lamp
    // 8 = decor
    // 9 = [default]
    // 10 = bookcase
    // 11 = table
    // 12 = rug
    // 13 = window
    // 14 = fireplace
    // 15 = bed...
    // 16 = torch
    // 17 = sconce
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self {
        let type_: &str = value_split[1usize];

        let (source_rectangle_width, source_rectangle_height): (u8, u8) = match value_split[2usize]
        {
            "-1" => {
                // Furniture.getDefaultSourceRectForType
                match type_ {
                    "chair" | "decor" | "window" | "torch" | "sconce" => (1u8, 2u8),
                    "bench" | "armchair" | "dresser" | "painting" => (2u8, 2u8),
                    "couch" | "rug" => (3u8, 2u8),
                    "long table" => (5u8, 3u8),
                    "lamp" => (1u8, 3u8),
                    "bookcase" | "table" => (2u8, 3u8),
                    "fireplace" => (2u8, 5u8),
                    _ => panic!("{}", type_),
                }
            }
            source_rectangle => {
                let mut source_rectangle_split: Split<&str> = source_rectangle.split(" ");
                (
                    source_rectangle_split
                        .next()
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                    source_rectangle_split
                        .next()
                        .unwrap()
                        .parse::<u8>()
                        .unwrap(),
                )
            }
        };

        let (bounding_box_width, bounding_box_height): (u8, u8) = match value_split[2usize] {
            "-1" => {
                // Furniture.getDefaultBoundingBoxForType
                match type_ {
                    "chair" | "lamp" | "decor" | "torch" => (1u8, 1u8),
                    "bench" | "armchair" | "dresser" | "bookcase" | "fireplace" => (2u8, 1u8),
                    "couch" => (3u8, 1u8),
                    "long table" => (5u8, 2u8),
                    "painting" | "table" => (2u8, 2u8),
                    "rug" => (3u8, 2u8),
                    "window" | "sconce" => (1u8, 2u8),
                    _ => panic!("{}", type_),
                }
            }
            bounding_box => {
                let mut bounding_box_split: Split<&str> = bounding_box.split(" ");
                (
                    bounding_box_split.next().unwrap().parse::<u8>().unwrap(),
                    bounding_box_split.next().unwrap().parse::<u8>().unwrap(),
                )
            }
        };

        Self {
            name: value_split[0usize].to_string(),
            type_: type_.to_string(),
            source_rectangle_width,
            source_rectangle_height,
            bounding_box_width,
            bounding_box_height,
            rotations: value_split[4usize].parse::<u8>().unwrap(),
            price: value_split[5usize].parse::<u32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct ClothingInformation {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub male_index: u16,
    pub female_index: u16,
    pub price: u32,
    pub rgb: (u8, u8, u8),
    pub dyeable: bool,
    pub type_: String,
}

impl FromValueSplit for ClothingInformation {
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self {
        let male_index: u16 = value_split[3usize].parse::<u16>().unwrap();
        let female_index: u16 = match value_split[4usize] {
            "-1" => male_index,
            _ => value_split[4usize].parse::<u16>().unwrap(),
        };

        let mut rgb_split: Split<&str> = value_split[6usize].split(" ");

        Self {
            name: value_split[0usize].to_string(),
            display_name: value_split[1usize].to_string(),
            description: value_split[2usize].to_string(),
            male_index,
            female_index,
            price: value_split[5usize].parse::<u32>().unwrap(),
            rgb: (
                rgb_split.next().unwrap().parse::<u8>().unwrap(),
                rgb_split.next().unwrap().parse::<u8>().unwrap(),
                rgb_split.next().unwrap().parse::<u8>().unwrap(),
            ),
            dyeable: value_split[7usize].parse::<bool>().unwrap(),
            type_: value_split[8usize].to_string(),
        }
    }
}

#[derive(Debug)]
struct Hats {
    pub name: String,
    pub description: String,
    pub hair_draw_type: u8,
    pub ignore_hairstyle_offset: bool,
    pub is_prismatic: bool,
}

impl FromValueSplit for Hats {
    fn from_value_split(id: u16, value_split: &Vec<&str>) -> Self {
        let hair_draw_type: u8 = match value_split[2usize] {
            "true" => 0u8,
            "false" => 1u8,
            "hide" => 2u8,
            _ => panic!(),
        };

        let mut is_prismatic = false;
        if value_split.len() > 4 {
            for special_tag in value_split[4usize].split(" ") {
                match special_tag {
                    "Prismatic" => {
                        is_prismatic = true;
                    }
                    _ => {}
                }
            }
        }

        Self {
            name: value_split[0usize].to_string(),
            description: value_split[1usize].to_string(),
            hair_draw_type,
            ignore_hairstyle_offset: value_split[3usize].parse::<bool>().unwrap(),
            is_prismatic,
        }
    }
}

fn load<T: Debug + FromValueSplit>(
    out_file: &mut BufWriter<File>,
    path: &Path,
    constant_name: &str,
    struct_name: &str,
) {
    let file: File = File::open(path).unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();

    let mut map: HashMap<u16, T> = HashMap::new();

    for (key, value) in json.get("content").unwrap().as_object().unwrap() {
        let id: u16 = match key.parse::<u16>() {
            Ok(key) => key,
            Err(_) => u16::MAX - (-key.parse::<i16>().unwrap() as u16), // This is very much a hack, but changing everything to i16 would require major changes.
        };
        let value_split: Vec<&str> = value.as_str().unwrap().split("/").collect();
        map.insert(
            // Clothing has some negative keys.
            id,
            T::from_value_split(id, &value_split),
        );
    }

    let entries: Vec<(u16, String)> = map
        .iter()
        .map(|(key, value)| (*key, format!("{:?}", value)))
        .collect();
    let mut builder: phf_codegen::Map<u16> = phf_codegen::Map::new();
    for (key, value) in &entries {
        builder.entry(*key, value);
    }

    writeln!(
        out_file,
        "pub static {}: phf::Map<u16, {}> = {};",
        constant_name,
        struct_name,
        builder.build(),
    )
    .unwrap();
}
fn main() {
    let out_path: PathBuf = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut out_file: BufWriter<File> = BufWriter::new(File::create(&out_path).unwrap());

    let object_information_path: &Path = Path::new("assets/ObjectInformation.json");
    load::<ObjectInformation>(
        &mut out_file,
        object_information_path,
        "OBJECT_INFORMATION",
        "ObjectInformation",
    );

    let big_craftables_information_path: &Path = Path::new("assets/BigCraftablesInformation.json");
    load::<BigCraftablesInformation>(
        &mut out_file,
        big_craftables_information_path,
        "BIG_CRAFTABLES_INFORMATION",
        "BigCraftablesInformation",
    );

    let furniture_path: &Path = Path::new("assets/Furniture.json");
    load::<Furniture>(&mut out_file, furniture_path, "FURNITURE", "Furniture");

    let clothing_information_path: &Path = Path::new("assets/ClothingInformation.json");
    load::<ClothingInformation>(
        &mut out_file,
        clothing_information_path,
        "CLOTHING_INFORMATION",
        "ClothingInformation",
    );

    let hats_path: &Path = Path::new("assets/hats.json");
    load::<Hats>(&mut out_file, hats_path, "HATS", "Hats");

    generate_cart_16(&mut out_file);
}

// --- Stardew 1.6 traveling-cart codegen (parsed from assets/Objects.json) ---
// Mirrors the verified stardew-seed-cracker model: assigns each object a position
// in C# dictionary order (preserve_order), classifies eligibility for the cart's
// RANDOM_ITEMS shuffle, and emits the fully-eligible pool.
fn generate_cart_16(out_file: &mut BufWriter<File>) {
    use xxhash_rust::xxh32::xxh32;

    let objects_file: File = File::open(Path::new("assets/Objects.json")).unwrap();
    let objects_json: serde_json::Value = serde_json::from_reader(objects_file).unwrap();
    let objects = objects_json.as_object().unwrap();

    let mut enumeration_entries: Vec<String> = Vec::new();
    let mut eligible_objects: Vec<(u16, String, u16)> = Vec::new();
    let mut all_objects: Vec<String> = Vec::new();
    let mut eligible_index: u16 = 0;

    for (key, value) in objects.iter() {
        let name = value.get("Name").unwrap().as_str().unwrap();
        let price = value.get("Price").unwrap().as_i64().unwrap() as i32;
        let typ = value.get("Type").unwrap().as_str().unwrap();
        let category = value.get("Category").unwrap().as_i64().unwrap() as i32;
        let exclude = value
            .get("ExcludeFromRandomSale")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Every object (numeric or string key) occupies a slot in the shuffle and
        // consumes one RNG draw, so emit a full ordered row for generic get_random_items.
        all_objects.push(format!(
            "ObjectRow16 {{ id: {}, name: {:?}, price: {}, offlimits: {}, category: {}, blocked_type: {} }}",
            key.parse::<u16>().unwrap_or(u16::MAX),
            name,
            price.max(0) as u32,
            exclude,
            category as i16,
            typ == "Quest" || typ == "Minerals" || typ == "Arch",
        ));

        // Layer 1 (RANDOM_ITEMS): numeric key in [2, 789], not excluded, price > 0.
        // Non-numeric keys still consume an RNG draw in the game's shuffle, so they
        // must remain in the enumeration as Ineligible.
        let id = key.parse::<u16>().ok();
        let passes_random_items =
            id.is_some_and(|id| (2..=789).contains(&id)) && !exclude && price > 0;
        if !passes_random_items {
            enumeration_entries.push("EnumerationEntry { class: ObjectClass::Ineligible }".into());
            continue;
        }
        let id = id.unwrap();

        // Layer 2 (PerItemCondition): Category < 0 && != -999, Type not Quest/Minerals/Arch.
        let passes_condition = category < 0
            && category != -999
            && typ != "Quest"
            && typ != "Minerals"
            && typ != "Arch";
        if !passes_condition {
            enumeration_entries
                .push("EnumerationEntry { class: ObjectClass::Intermediate }".into());
            continue;
        }

        enumeration_entries.push(format!(
            "EnumerationEntry {{ class: ObjectClass::FullyEligible({}) }}",
            eligible_index
        ));
        eligible_objects.push((id, name.to_string(), price as u16));
        eligible_index += 1;
    }

    writeln!(
        out_file,
        "pub static OBJECT_ENUMERATION: [EnumerationEntry; {}] = [",
        enumeration_entries.len()
    )
    .unwrap();
    for entry in &enumeration_entries {
        writeln!(out_file, "    {},", entry).unwrap();
    }
    writeln!(out_file, "];").unwrap();

    writeln!(
        out_file,
        "pub static ELIGIBLE_OBJECTS: [EligibleObject; {}] = [",
        eligible_objects.len()
    )
    .unwrap();
    for (id, name, price) in &eligible_objects {
        writeln!(
            out_file,
            "    EligibleObject {{ id: {}, name: {:?}, price: {} }},",
            id, name, price
        )
        .unwrap();
    }
    writeln!(out_file, "];").unwrap();

    writeln!(
        out_file,
        "pub static ALL_OBJECTS_16: [ObjectRow16; {}] = [",
        all_objects.len()
    )
    .unwrap();
    for row in &all_objects {
        writeln!(out_file, "    {},", row).unwrap();
    }
    writeln!(out_file, "];").unwrap();

    let synced_keys = [
        ("HASH_CART_FEZ", "cart_fez"),
        ("HASH_CART_COFFEE_BEAN", "cart_coffee_bean"),
        ("HASH_CART_RARECROW", "cart_rarecrow"),
        ("HASH_CART_RETRO_CATALOGUE", "cart_retroCatalogue"),
        ("HASH_CART_JOJA_CATALOGUE", "cart_jojaCatalogue"),
        ("HASH_CART_JUNIMO_CATALOGUE", "cart_junimoCatalogue"),
        ("HASH_TRAVELER_SKILL_BOOK", "travelerSkillBook"),
        ("HASH_TEASET", "teaset"),
        ("HASH_KROBUS_BREAD", "krobus_bread"),
    ];
    for (const_name, key_str) in &synced_keys {
        let hash = xxh32(key_str.as_bytes(), 0) as i32;
        writeln!(out_file, "pub const {}: i32 = {}i32;", const_name, hash).unwrap();
    }

    writeln!(
        out_file,
        "pub const TOTAL_OBJECTS: usize = {};",
        enumeration_entries.len()
    )
    .unwrap();
    writeln!(
        out_file,
        "pub const TOTAL_ELIGIBLE: usize = {};",
        eligible_objects.len()
    )
    .unwrap();

    // Furniture pool (game dictionary order) for the cart's single furniture pick.
    let furn_file: File = File::open(Path::new("assets/Furniture16.json")).unwrap();
    let furn_json: serde_json::Value = serde_json::from_reader(furn_file).unwrap();
    let furn = furn_json.as_array().unwrap();
    writeln!(
        out_file,
        "pub static FURNITURE_16: [Furniture16; {}] = [",
        furn.len()
    )
    .unwrap();
    for f in furn {
        // Non-numeric (string) IDs still consume an RNG draw in the shuffle but are
        // never selectable; store a sentinel id so the runtime filter excludes them.
        let id = f
            .get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as u16)
            .unwrap_or(u16::MAX);
        let name = f.get("name").unwrap().as_str().unwrap();
        let price = f.get("price").unwrap().as_i64().unwrap() as u32;
        let offlimits = f.get("offlimits").unwrap().as_bool().unwrap();
        writeln!(
            out_file,
            "    Furniture16 {{ id: {}, name: {:?}, price: {}, offlimits: {} }},",
            id, name, price, offlimits
        )
        .unwrap();
    }
    writeln!(out_file, "];").unwrap();
    writeln!(
        out_file,
        "pub const TOTAL_FURNITURE_16: usize = {};",
        furn.len()
    )
    .unwrap();

    // Shirts pool (game dictionary order) for Sandy's daily shirt pick.
    let shirts_file: File = File::open(Path::new("assets/Shirts16.json")).unwrap();
    let shirts_json: serde_json::Value = serde_json::from_reader(shirts_file).unwrap();
    let shirts = shirts_json.as_array().unwrap();
    writeln!(
        out_file,
        "pub static SHIRTS_16: [ShirtRow16; {}] = [",
        shirts.len()
    )
    .unwrap();
    for s in shirts {
        let id = s
            .get("id")
            .and_then(|v| v.as_i64())
            .map(|v| v as u16)
            .unwrap_or(u16::MAX);
        let name = s.get("name").unwrap().as_str().unwrap();
        let price = s.get("price").unwrap().as_i64().unwrap() as u32;
        let offlimits = s.get("offlimits").unwrap().as_bool().unwrap();
        writeln!(
            out_file,
            "    ShirtRow16 {{ id: {}, name: {:?}, price: {}, offlimits: {} }},",
            id, name, price, offlimits
        )
        .unwrap();
    }
    writeln!(out_file, "];").unwrap();
    writeln!(
        out_file,
        "pub const TOTAL_SHIRTS_16: usize = {};",
        shirts.len()
    )
    .unwrap();
}
