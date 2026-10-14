//! Stardew 1.6 traveling-cart core (platform-aware), ported from the verified
//! stardew-seed-cracker. Produces the 10 random stock items and the Year-1
//! Red Cabbage Seeds guarantee. Furniture / seasonal / named items build on top.
use std::cmp::max;

use crate::codegen::{
    Furniture16, ObjectClass, ObjectRow16, EligibleObject, ALL_OBJECTS_16, ELIGIBLE_OBJECTS,
    FURNITURE_16, HASH_CART_COFFEE_BEAN, HASH_CART_FEZ, HASH_CART_JOJA_CATALOGUE,
    HASH_CART_JUNIMO_CATALOGUE, HASH_CART_RARECROW, HASH_CART_RETRO_CATALOGUE, HASH_TEASET,
    HASH_TRAVELER_SKILL_BOOK, OBJECT_ENUMERATION, SHIRTS_16, TOTAL_ELIGIBLE, TOTAL_OBJECTS,
};
use crate::configuration::Platform;
use crate::prng16::{get_prng, Prng};
use crate::xxhash::{shop_seed, synced_random_seed};

pub const STOCK_QUANTITY: usize = 10;

/// Object IDs referenced by the Year-1 guarantee / Red Cabbage predictions.
pub const RED_CABBAGE: u16 = 266;
pub const RED_CABBAGE_SEEDS: u16 = 485;

/// Books the traveling merchant can offer (Data/Shops traveler skill book).
const SKILL_BOOKS: [&str; 5] = [
    "Stardew Valley Almanac",
    "Bait And Bobber",
    "Woodcutter's Weekly",
    "Mining Monthly",
    "Combat Quarterly",
];

pub struct GeneratedItem {
    pub eligible_index: u16,
    pub price: u16,
    pub quantity: u8,
}

impl GeneratedItem {
    pub fn id(&self) -> u16 {
        ELIGIBLE_OBJECTS[self.eligible_index as usize].id
    }
    pub fn name(&self) -> &'static str {
        ELIGIBLE_OBJECTS[self.eligible_index as usize].name
    }
}

/// Absolute day index (1-based) for a 1-based year, 0-based season, 1-based day.
pub fn days_played(year: i32, season: u8, day: i32) -> i32 {
    (year - 1) * 112 + season as i32 * 28 + day
}

fn eligible_price(id: u16) -> u16 {
    ELIGIBLE_OBJECTS
        .iter()
        .find(|o: &&EligibleObject| o.id == id)
        .map(|o| o.price)
        .unwrap_or(0)
}

/// The travelling cart's 10 random stock items for a given shop seed.
pub fn generate_stock(platform: Platform, seed: i32) -> [GeneratedItem; STOCK_QUANTITY] {
    let mut prng = get_prng(platform, seed);
    generate_stock_with(&mut prng)
}

/// Same as `generate_stock` but continues an existing PRNG (so later rolls —
/// guarantee, furniture, seasonal — stay in sequence).
fn generate_stock_with(prng: &mut Box<dyn Prng>) -> [GeneratedItem; STOCK_QUANTITY] {
    // Phase A: assign a Next() key to every object (dict order), keep eligible ones.
    let mut keyed: Vec<(i32, u16)> = Vec::with_capacity(TOTAL_ELIGIBLE);
    for pos in 0..TOTAL_OBJECTS {
        let key = prng.next();
        if let ObjectClass::FullyEligible(idx) = OBJECT_ENUMERATION[pos].class {
            keyed.push((key, idx));
        }
    }
    keyed.sort_by_key(|&(key, _)| key);

    // Phase B: price and quantity rolls, one per slot in sort-key (= UI slot) order.
    std::array::from_fn(|slot| {
        let set_idx = prng.next_max(10);
        let mult_idx = prng.next_max(3);
        let qty_roll = prng.next_double();

        let elig_idx = keyed[slot].1;
        let base_price = ELIGIBLE_OBJECTS[elig_idx as usize].price;
        let set_price = (set_idx as u16 + 1) * 100;
        let multiplied = base_price * [3u16, 4u16, 5u16][mult_idx as usize];
        GeneratedItem {
            eligible_index: elig_idx,
            price: max(set_price, multiplied),
            quantity: if qty_roll < 0.1 { 5 } else { 1 },
        }
    })
}

/// The cart's single furniture pick: shuffle the furniture pool by Next() key and
/// take the lowest eligible (price > 0, not off-limits, id in [0, 1612]).
fn pick_furniture(prng: &mut Box<dyn Prng>) -> &'static Furniture16 {
    let mut keyed: Vec<(i32, usize)> = Vec::with_capacity(FURNITURE_16.len());
    for (i, f) in FURNITURE_16.iter().enumerate() {
        let key = prng.next();
        if f.price > 0 && !f.offlimits && f.id <= 1612 {
            keyed.push((key, i));
        }
    }
    keyed.sort_by_key(|&(key, _)| key);
    &FURNITURE_16[keyed[0].1]
}

/// A SYNCED_RANDOM conditional item rolls present with `prob` on a given day.
fn synced_present(platform: Platform, uid: u64, dp: i32, key_hash: i32, prob: f64) -> bool {
    let mut rng = get_prng(platform, synced_random_seed(key_hash, uid, dp));
    rng.next_double() < prob
}

/// Generic `ItemQueryResolver.RANDOM_ITEMS` over an object id range — the shared
/// shuffle used by the cart and the other 1.6 shops. Returns indices into
/// ALL_OBJECTS_16 for the first `how_many` items passing the requested filters.
pub fn get_random_items(
    prng: &mut Box<dyn Prng>,
    min: u16,
    max: u16,
    require_price: bool,
    is_random_sale: bool,
    do_category_checks: bool,
    how_many: usize,
) -> Vec<usize> {
    let mut keyed: Vec<(i32, usize)> = Vec::new();
    for (i, o) in ALL_OBJECTS_16.iter().enumerate() {
        let key = prng.next();
        if o.id == u16::MAX {
            continue; // non-numeric key (int.TryParse fails)
        }
        if require_price && o.price == 0 {
            continue;
        }
        if is_random_sale && o.offlimits {
            continue;
        }
        if o.id >= min && o.id <= max {
            keyed.push((key, i));
        }
    }
    keyed.sort_by_key(|&(key, _)| key);

    let mut selected: Vec<usize> = Vec::new();
    for (_, i) in keyed {
        let o: &ObjectRow16 = &ALL_OBJECTS_16[i];
        if do_category_checks && (o.category >= 0 || o.category == -999) {
            continue;
        }
        if do_category_checks && o.blocked_type {
            continue;
        }
        selected.push(i);
        if selected.len() >= how_many {
            break;
        }
    }
    selected
}

/// RANDOM_ITEMS shuffle over the shirts pool (no category checks) — Sandy's pick.
/// Returns the index into SHIRTS_16 of the lowest-key eligible shirt in [min, max].
pub fn get_random_shirt(prng: &mut Box<dyn Prng>, min: u16, max: u16) -> usize {
    let mut keyed: Vec<(i32, usize)> = Vec::new();
    for (i, s) in SHIRTS_16.iter().enumerate() {
        let key = prng.next();
        if s.id == u16::MAX || s.price == 0 || s.offlimits {
            continue;
        }
        if s.id >= min && s.id <= max {
            keyed.push((key, i));
        }
    }
    keyed.sort_by_key(|&(key, _)| key);
    keyed[0].1
}

/// Numeric wallpaper/flooring shuffle (mp getRandomWallFloor): ids `min..=max+extra`
/// each get a Next() key; return the first `how_many` with id <= max and not excluded.
pub fn get_random_wall_floor(
    prng: &mut Box<dyn Prng>,
    min: u16,
    max: u16,
    extra: u16,
    exclude: &[u16],
    how_many: usize,
) -> Vec<u16> {
    let mut keyed: Vec<(i32, u16)> = Vec::new();
    for id in min..=(max + extra) {
        let key = prng.next();
        keyed.push((key, id));
    }
    keyed.sort_by_key(|&(key, _)| key);

    let mut selected: Vec<u16> = Vec::new();
    for (_, id) in keyed {
        if exclude.contains(&id) || id > max {
            continue;
        }
        selected.push(id);
        if selected.len() >= how_many {
            break;
        }
    }
    selected
}

/// Convenience: stock for a calendar date.
pub fn stock_for_date(platform: Platform, uid: u64, year: i32, season: u8, day: i32) -> [GeneratedItem; STOCK_QUANTITY] {
    generate_stock(platform, shop_seed(days_played(year, season, day), uid))
}

/// Cart visits the forest on Fridays and Sundays.
pub fn is_regular_cart_day(days_played: i32) -> bool {
    matches!((days_played - 1).rem_euclid(7), 4 | 6)
}

/// Desert Festival (Spring 15-17) and Night Market (Winter 15-17) add cart days.
/// Normalised to day-of-year so it holds for every year, not just Year 1.
pub fn is_festival_cart_day(days_played: i32) -> bool {
    let day_of_year = (days_played - 1).rem_euclid(112) + 1;
    matches!(day_of_year, 15 | 16 | 17 | 99 | 100 | 101)
}

/// Any day the cart is reachable (regular forest visit or a festival).
pub fn is_cart_day(days_played: i32) -> bool {
    is_regular_cart_day(days_played) || is_festival_cart_day(days_played)
}

/// Stock for an absolute `days_played` value.
pub fn stock_for_days_played(platform: Platform, uid: u64, days_played: i32) -> [GeneratedItem; STOCK_QUANTITY] {
    generate_stock(platform, shop_seed(days_played, uid))
}

/// Red Cabbage Seeds Year-1 guarantee: the game rolls the initial visit count from
/// `new Random(12 * uniqueIDForThisGame).Next(2, 31)` (platform PRNG), then forces the
/// seeds into the cart on the Fri/Sun visit where the countdown reaches zero.
pub fn original_guarantee(platform: Platform, uid: u64) -> i32 {
    let seed = (12i64 * uid as i64) as i32;
    let mut rng = get_prng(platform, seed);
    rng.next_min_max(2..31)
}

/// Remaining guarantee countdown on cart day `dp` (reaches 0 on the forced day).
fn visits_now(dp: i32, original: i32) -> i32 {
    let mut v = original - dp / 7 - (dp + 2) / 7;
    if dp >= 99 {
        v -= 1;
    }
    if dp >= 100 {
        v -= 1;
    }
    if dp >= 101 {
        v -= 1;
    }
    v
}

/// `days_played` (within Year 1) on which the guarantee forces Red Cabbage Seeds.
/// Evaluated on every cart day, including Night Market / Desert Festival days.
pub fn guarantee_day(original: i32) -> Option<i32> {
    for dp in 1..=112 {
        if is_cart_day(dp) && visits_now(dp, original) == 0 {
            return Some(dp);
        }
    }
    None
}

/// Year-1 Red Cabbage summary for the UI.
pub struct Year1RedCabbage {
    /// Cart days (days_played) where Red Cabbage (266) is randomly in stock.
    pub cabbage_days: Vec<i32>,
    /// Cart days where Red Cabbage Seeds (485) are randomly in stock.
    pub seed_days: Vec<i32>,
    /// The guarantee's initial rolled visit count.
    pub original: i32,
    /// The day the guarantee forces Red Cabbage Seeds into stock.
    pub guarantee_day: Option<i32>,
}

pub fn year1_red_cabbage(platform: Platform, uid: u64) -> Year1RedCabbage {
    let mut cabbage_days = Vec::new();
    let mut seed_days = Vec::new();
    for dp in 1..=112 {
        if !is_cart_day(dp) {
            continue;
        }
        let ids: Vec<u16> = stock_for_days_played(platform, uid, dp)
            .iter()
            .map(|it| it.id())
            .collect();
        if ids.contains(&RED_CABBAGE) {
            cabbage_days.push(dp);
        }
        if ids.contains(&RED_CABBAGE_SEEDS) {
            seed_days.push(dp);
        }
    }
    let original = original_guarantee(platform, uid);
    Year1RedCabbage {
        cabbage_days,
        seed_days,
        original,
        guarantee_day: guarantee_day(original),
    }
}

/// One displayable row of a day's cart stock.
pub struct CartLine {
    pub category: String,
    pub name: String,
    pub qty: String,
    pub price: String,
    pub note: Option<&'static str>,
}

/// Full cart stock for a single day, in the game's RNG-consumption order:
/// 10 random items -> Year-1 guarantee (if due) -> furniture -> seasonal special
/// -> skill book, plus the independent SYNCED_RANDOM specials.
pub fn day_cart(platform: Platform, uid: u64, dp: i32) -> Vec<CartLine> {
    let season = (((dp - 1) / 28) % 4) as u8;
    let year = (dp - 1) / 112 + 1;
    let mut lines: Vec<CartLine> = Vec::new();

    let mut prng = get_prng(platform, shop_seed(dp, uid));
    let items = generate_stock_with(&mut prng);
    let mut seen_rare_seed = false;
    for (i, item) in items.iter().enumerate() {
        if item.name() == "Rare Seed" {
            seen_rare_seed = true;
        }
        lines.push(CartLine {
            category: format!("Item {}", i + 1),
            name: item.name().to_string(),
            qty: format!("x{}", item.quantity),
            price: format!("{}g", item.price),
            note: None,
        });
    }

    // Year-1 Red Cabbage Seeds guarantee (consumes RNG only on the forced day).
    let original = original_guarantee(platform, uid);
    if year == 1 && is_cart_day(dp) && visits_now(dp, original) == 0 {
        let base = eligible_price(RED_CABBAGE_SEEDS);
        let set_price = (prng.next_max(10) as u16 + 1) * 100;
        let multiplied = base * [3u16, 4u16, 5u16][prng.next_max(3) as usize];
        let qty = if prng.next_double() < 0.1 { 5 } else { 1 };
        lines.push(CartLine {
            category: "Y1 Guarantee".to_string(),
            name: "Red Cabbage Seeds".to_string(),
            qty: format!("x{}", qty),
            price: format!("{}g", max(set_price, multiplied)),
            note: None,
        });
    }

    // Furniture.
    let furniture = pick_furniture(&mut prng);
    let furniture_price = (prng.next_min_max(1..11) as u32) * 250;
    lines.push(CartLine {
        category: "Furniture".to_string(),
        name: furniture.name.to_string(),
        qty: "x1".to_string(),
        price: format!("{}g", furniture_price),
        note: None,
    });

    // Seasonal Rare Seed (Spring/Summer) when not already among the 10.
    if season < 2 && !seen_rare_seed {
        let qty = if prng.next_double() < 0.1 { 5 } else { 1 };
        lines.push(CartLine {
            category: "Seasonal".to_string(),
            name: "Rare Seed".to_string(),
            qty: format!("x{}", qty),
            price: "1000g".to_string(),
            note: None,
        });
    }

    // Skill book: present via its SYNCED roll; the book itself is a main-RNG pick.
    if synced_present(platform, uid, dp, HASH_TRAVELER_SKILL_BOOK, 0.05) {
        let idx = prng.next_max(SKILL_BOOKS.len() as i32) as usize;
        lines.push(CartLine {
            category: "Skill Book".to_string(),
            name: SKILL_BOOKS[idx].to_string(),
            qty: "x1".to_string(),
            price: "6000g".to_string(),
            note: None,
        });
    }

    // Independent SYNCED_RANDOM specials (order irrelevant — separate seeds).
    if season >= 2 {
        if synced_present(platform, uid, dp, HASH_CART_RARECROW, 0.4) {
            lines.push(CartLine {
                category: "Seasonal".to_string(),
                name: "Rarecrow (Snowman)".to_string(),
                qty: "x1".to_string(),
                price: "4000g".to_string(),
                note: None,
            });
        }
        if synced_present(platform, uid, dp, HASH_CART_COFFEE_BEAN, 0.25) {
            lines.push(CartLine {
                category: "Special".to_string(),
                name: "Coffee Bean".to_string(),
                qty: "x1".to_string(),
                price: "2500g".to_string(),
                note: None,
            });
        }
    }
    if synced_present(platform, uid, dp, HASH_CART_FEZ, 0.1) {
        lines.push(CartLine {
            category: "Special".to_string(),
            name: "Red Fez".to_string(),
            qty: "x1".to_string(),
            price: "8000g".to_string(),
            note: None,
        });
    }
    if synced_present(platform, uid, dp, HASH_CART_RETRO_CATALOGUE, 0.1) {
        lines.push(CartLine {
            category: "Catalogue".to_string(),
            name: "Retro Catalogue".to_string(),
            qty: "x1".to_string(),
            price: "110000g".to_string(),
            note: None,
        });
    }
    if synced_present(platform, uid, dp, HASH_CART_JOJA_CATALOGUE, 0.1) {
        lines.push(CartLine {
            category: "Catalogue".to_string(),
            name: "Joja Catalogue".to_string(),
            qty: "x1".to_string(),
            price: "30000g".to_string(),
            note: Some("if Community Center completed"),
        });
    }
    if synced_present(platform, uid, dp, HASH_CART_JUNIMO_CATALOGUE, 0.1) {
        lines.push(CartLine {
            category: "Catalogue".to_string(),
            name: "Junimo Catalogue".to_string(),
            qty: "x1".to_string(),
            price: "70000g".to_string(),
            note: Some("if CC or Joja route completed"),
        });
    }
    if year >= 25 && synced_present(platform, uid, dp, HASH_TEASET, 0.05) {
        lines.push(CartLine {
            category: "Special".to_string(),
            name: "Tea Set".to_string(),
            qty: "∞".to_string(),
            price: "1000000g".to_string(),
            note: None,
        });
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verified against stardew-seed-cracker `simulate` (real Switch 1.6.15 cart).
    #[test]
    fn switch_spring26_y1_matches_real_cart() {
        let items = stock_for_date(Platform::Switch, 450982496, 1, 0, 26);
        let expected: [(&str, u16, u8); 10] = [
            ("Walleye", 525, 1),
            ("Maple Bar", 1200, 1),
            ("Miner's Treat", 1000, 1),
            ("Fall Seeds", 800, 1),
            ("Pizza", 900, 1),
            ("Artichoke", 900, 1),
            ("Fairy Seeds", 900, 1),
            ("Poppy", 700, 1),
            ("Trout Soup", 900, 1),
            ("Brick Floor", 400, 5),
        ];
        for (item, (name, price, qty)) in items.iter().zip(expected.iter()) {
            assert_eq!(item.name(), *name);
            assert_eq!(item.price, *price);
            assert_eq!(item.quantity, *qty);
        }
    }

    // Both cracked candidates share cart stock (depends only on floor(uid/2)).
    #[test]
    fn candidate_seeds_share_stock() {
        let a = stock_for_date(Platform::Switch, 450982496, 1, 0, 26);
        let b = stock_for_date(Platform::Switch, 450982497, 1, 0, 26);
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.id(), y.id());
            assert_eq!(x.price, y.price);
        }
    }

    // Platform must change the outcome (JKISS vs MsCorLibRandom).
    #[test]
    fn platform_changes_stock() {
        let sw = stock_for_date(Platform::Switch, 450982496, 1, 0, 26);
        let pc = stock_for_date(Platform::PC, 450982496, 1, 0, 26);
        let same = sw.iter().zip(pc.iter()).all(|(a, b)| a.id() == b.id());
        assert!(!same, "Switch and PC stock should differ for the same seed");
    }

    // Verified against the `red_cabbage` scan bin.
    #[test]
    fn switch_guarantee_day() {
        let original = original_guarantee(Platform::Switch, 450982496);
        assert_eq!(original, 19);
        // Fall 12, Year 1 = day 2*28 + 12 = 68.
        assert_eq!(guarantee_day(original), Some(days_played(1, 2, 12)));
    }

    // Guarantee rolls that land on Night Market festival days must be reported
    // (regression for L-r1-04 — festival days were previously skipped).
    #[test]
    fn guarantee_on_night_market_days() {
        assert_eq!(guarantee_day(29), Some(99)); // Winter 15 (Night Market)
        assert_eq!(guarantee_day(30), Some(100)); // Winter 16 (Night Market)
    }

    // Festival cart days must be recognised in every year, not just Year 1
    // (regression for L-r2-02).
    #[test]
    fn festival_cart_days_are_annual() {
        assert!(is_festival_cart_day(days_played(2, 0, 15))); // Y2 Desert Festival
        assert!(is_festival_cart_day(days_played(2, 3, 15))); // Y2 Night Market
        assert!(is_cart_day(days_played(3, 3, 17))); // Y3 Night Market
        assert!(!is_festival_cart_day(days_played(2, 0, 18))); // not a festival day
    }

    // The full day_cart keeps the verified 10 items first, then adds furniture etc.
    #[test]
    fn day_cart_preserves_items_and_adds_furniture() {
        let lines = day_cart(Platform::Switch, 450982496, days_played(1, 0, 26));
        assert_eq!(lines[0].name, "Walleye");
        assert_eq!(lines[0].price, "525g");
        assert_eq!(lines[9].name, "Brick Floor");
        assert_eq!(lines[9].qty, "x5");
        assert!(lines.iter().any(|l| l.category == "Furniture"));
    }

    // Cross-check furniture against the mp CSRandom harness (_verify/cart.cjs),
    // which predicts "Globe" for this seed/date on PC.
    #[test]
    fn pc_furniture_matches_mp_harness() {
        let lines = day_cart(Platform::PC, 123456789, days_played(1, 0, 5));
        let furniture = lines.iter().find(|l| l.category == "Furniture").unwrap();
        assert_eq!(furniture.name, "Globe");
    }

    // The generic get_random_items must reproduce the verified cart's 10 items when
    // given the cart's own range/filters — validating it for the other shops.
    #[test]
    fn get_random_items_reproduces_cart() {
        let mut prng = get_prng(Platform::Switch, shop_seed(days_played(1, 0, 26), 450982496));
        let picks = get_random_items(&mut prng, 2, 789, true, true, true, 10);
        let names: Vec<&str> = picks.iter().map(|&i| ALL_OBJECTS_16[i].name).collect();
        assert_eq!(
            names,
            vec![
                "Walleye",
                "Maple Bar",
                "Miner's Treat",
                "Fall Seeds",
                "Pizza",
                "Artichoke",
                "Fairy Seeds",
                "Poppy",
                "Trout Soup",
                "Brick Floor"
            ]
        );
    }
}
