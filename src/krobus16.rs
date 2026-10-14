//! Stardew 1.6 Krobus (Sewer shadow shop), platform-aware. Logic from mp's
//! predictKrobus / StardewValley.Locations.Sewer.getShadowShopStock(). Uid-reliable
//! (uses uniqueIDForThisGame only, no player multiplayer ID).
use crate::cart16::get_random_items;
use crate::codegen::{ALL_OBJECTS_16, HASH_KROBUS_BREAD};
use crate::configuration::Platform;
use crate::prng16::get_prng;
use crate::xxhash::{create_random_seed, shop_seed};

pub struct KrobusItem {
    pub id: u16,
    pub name: &'static str,
    pub qty: u8,
    pub price: u32,
}

fn object_name(id: u16) -> &'static str {
    ALL_OBJECTS_16
        .iter()
        .find(|o| o.id == id)
        .map(|o| o.name)
        .unwrap_or("?")
}

/// Wednesday: a random fish (object id 698-708), 5 for 200g.
pub fn wednesday(platform: Platform, uid: u64, dp: i32) -> KrobusItem {
    let mut prng = get_prng(platform, shop_seed(dp, uid));
    let pick = get_random_items(&mut prng, 698, 708, true, true, false, 1);
    let o = &ALL_OBJECTS_16[pick[0]];
    KrobusItem {
        id: o.id,
        name: o.name,
        qty: 5,
        price: 200,
    }
}

/// Saturday: usually a random cooked dish (id 194-244); ~1.96% chance of Bread (216).
pub fn saturday(platform: Platform, uid: u64, dp: i32) -> KrobusItem {
    let mut prng = get_prng(platform, shop_seed(dp, uid));
    let mut bread = get_prng(platform, create_random_seed(HASH_KROBUS_BREAD, dp, 0, 0, 0));
    if bread.next_double() < 0.0196078431 {
        let price = 10 * prng.next_min_max(5..51) as u32;
        KrobusItem {
            id: 216,
            name: object_name(216),
            qty: 5,
            price,
        }
    } else {
        let pick = get_random_items(&mut prng, 194, 244, true, true, false, 1);
        let o = &ALL_OBJECTS_16[pick[0]];
        let price = 10 * prng.next_min_max(5..51) as u32;
        KrobusItem {
            id: o.id,
            name: o.name,
            qty: 5,
            price,
        }
    }
}

/// A week's Krobus offerings (Wednesday + Saturday) for any day in that week.
pub fn week(platform: Platform, uid: u64, dp: i32) -> (KrobusItem, KrobusItem) {
    let offset = 7 * ((dp - 1) / 7);
    (
        wednesday(platform, uid, offset + 3),
        saturday(platform, uid, offset + 6),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cart16::days_played;

    #[test]
    fn krobus_items_in_expected_ranges() {
        for dp in 1..=112 {
            let (wed, sat) = week(Platform::Switch, 450982496, dp);
            assert!((698..=708).contains(&wed.id), "Wed fish id {}", wed.id);
            assert_eq!(wed.price, 200);
            assert!(
                sat.id == 216 || (194..=244).contains(&sat.id),
                "Sat dish id {}",
                sat.id
            );
            assert!(sat.price >= 50 && sat.price <= 500);
        }
        // Platform must matter.
        let sw = week(Platform::Switch, 450982496, days_played(1, 0, 1));
        let pc = week(Platform::PC, 450982496, days_played(1, 0, 1));
        assert!(sw.0.id != pc.0.id || sw.1.id != pc.1.id);
    }

    // Exact golden values from the mp CSRandom harness (uid 123456789, PC).
    #[test]
    fn krobus_wednesday_matches_mp_harness() {
        assert_eq!(wednesday(Platform::PC, 123456789, 3).id, 705); // Albacore
        assert_eq!(wednesday(Platform::PC, 123456789, 5).id, 706); // Shad
    }
}
