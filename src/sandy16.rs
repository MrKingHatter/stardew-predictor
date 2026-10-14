//! Stardew 1.6 Sandy (Oasis) daily shirt, platform-aware and uid-reliable.
//! Logic from mp's predictSandy / GameLocation.sandyShopStock().
use crate::cart16::get_random_shirt;
use crate::codegen::SHIRTS_16;
use crate::configuration::Platform;
use crate::prng16::get_prng;
use crate::xxhash::shop_seed;

pub struct SandyShirt {
    pub id: u16,
    pub name: &'static str,
}

/// The shirt Sandy sells on the given absolute day.
pub fn shirt_for_day(platform: Platform, uid: u64, dp: i32) -> SandyShirt {
    let mut prng = get_prng(platform, shop_seed(dp, uid));
    let s = &SHIRTS_16[get_random_shirt(&mut prng, 1000, 1126)];
    SandyShirt {
        id: s.id,
        name: s.name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cart16::days_played;

    #[test]
    fn sandy_shirt_in_range() {
        for dp in 1..=112 {
            let s = shirt_for_day(Platform::Switch, 450982496, dp);
            assert!((1000..=1126).contains(&s.id), "shirt id {}", s.id);
        }
        let sw = shirt_for_day(Platform::Switch, 450982496, days_played(1, 0, 1));
        let pc = shirt_for_day(Platform::PC, 450982496, days_played(1, 0, 1));
        assert!(sw.id != pc.id);
    }

    // Exact golden values from the mp CSRandom harness (uid 123456789, PC).
    #[test]
    fn sandy_matches_mp_harness() {
        assert_eq!(shirt_for_day(Platform::PC, 123456789, 3).id, 1096);
        assert_eq!(shirt_for_day(Platform::PC, 123456789, 5).id, 1006);
    }
}
