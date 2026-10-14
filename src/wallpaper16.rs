//! Stardew 1.6 Pierre / Joja daily wallpaper + flooring, platform-aware and
//! uid-reliable. Logic from mp (getRandomWallFloor over Data/Shops ranges).
//! Pierre and Joja share the day seed; only their flooring ranges differ.
use crate::cart16::get_random_wall_floor;
use crate::configuration::Platform;
use crate::prng16::get_prng;
use crate::xxhash::shop_seed;

pub struct WallFloor {
    pub wallpaper: u16,
    pub flooring: u16,
}

pub fn pierre(platform: Platform, uid: u64, dp: i32) -> WallFloor {
    let mut rng = get_prng(platform, shop_seed(dp, uid));
    let wallpaper = get_random_wall_floor(&mut rng, 0, 111, 9, &[21], 1)[0];
    let flooring = 1 + get_random_wall_floor(&mut rng, 0, 55, 22, &[], 1)[0];
    WallFloor { wallpaper, flooring }
}

pub fn joja(platform: Platform, uid: u64, dp: i32) -> WallFloor {
    let mut rng = get_prng(platform, shop_seed(dp, uid));
    let wallpaper = get_random_wall_floor(&mut rng, 0, 111, 9, &[21], 1)[0];
    let flooring = 1 + get_random_wall_floor(&mut rng, 0, 39, 48, &[], 1)[0];
    WallFloor { wallpaper, flooring }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cart16::days_played;

    #[test]
    fn wall_floor_in_range_and_pierre_joja_share_wallpaper() {
        for dp in 1..=112 {
            let p = pierre(Platform::Switch, 450982496, dp);
            let j = joja(Platform::Switch, 450982496, dp);
            assert!(p.wallpaper <= 111 && p.wallpaper != 21);
            assert!(p.flooring >= 1 && p.flooring <= 56);
            assert!(j.flooring >= 1 && j.flooring <= 40);
            // Same seed => same wallpaper roll for both shops.
            assert_eq!(p.wallpaper, j.wallpaper);
        }
        let sw = pierre(Platform::Switch, 450982496, days_played(1, 0, 1));
        let pc = pierre(Platform::PC, 450982496, days_played(1, 0, 1));
        assert!(sw.wallpaper != pc.wallpaper || sw.flooring != pc.flooring);
    }

    // Exact golden values from the mp CSRandom harness (uid 123456789, PC).
    #[test]
    fn wall_floor_matches_mp_harness() {
        let p3 = pierre(Platform::PC, 123456789, 3);
        assert_eq!((p3.wallpaper, p3.flooring), (96, 20));
        let j3 = joja(Platform::PC, 123456789, 3);
        assert_eq!((j3.wallpaper, j3.flooring), (96, 20));
        let p5 = pierre(Platform::PC, 123456789, 5);
        assert_eq!((p5.wallpaper, p5.flooring), (6, 4));
        let j5 = joja(Platform::PC, 123456789, 5);
        assert_eq!((j5.wallpaper, j5.flooring), (6, 4));
    }
}
