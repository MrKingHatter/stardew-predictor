//! Stardew 1.6 shop RNG seeding (cross-platform, xxHash32-based), ported from the
//! verified stardew-seed-cracker. Matches `Utility.CreateRandomSeed`.
use xxhash_rust::xxh32::xxh32;

/// General `Utility.CreateRandomSeed(a,b,c,d,e)` (non-legacy): xxHash32 of the five
/// int seed components, each reduced mod int.MaxValue, packed little-endian.
pub fn create_random_seed(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let m = |x: i32| (x as f64 % 2147483647.0) as i32;
    let mut buf = [0u8; 20];
    buf[0..4].copy_from_slice(&m(a).to_le_bytes());
    buf[4..8].copy_from_slice(&m(b).to_le_bytes());
    buf[8..12].copy_from_slice(&m(c).to_le_bytes());
    buf[12..16].copy_from_slice(&m(d).to_le_bytes());
    buf[16..20].copy_from_slice(&m(e).to_le_bytes());
    xxh32(&buf, 0) as i32
}

/// Travelling-cart shop seed: `CreateRandomSeed(days_played, uniqueIDForThisGame / 2)`
/// = xxHash32(LE pack [days_played, uid/2, 0, 0, 0]).
pub fn shop_seed(days_played: i32, uid: u64) -> i32 {
    let dp = (days_played as f64 % 2147483647.0) as i32;
    let k = ((uid / 2) as f64 % 2147483647.0) as i32;
    let mut buf = [0u8; 20];
    buf[0..4].copy_from_slice(&dp.to_le_bytes());
    buf[4..8].copy_from_slice(&k.to_le_bytes());
    xxh32(&buf, 0) as i32
}

/// SYNCED_RANDOM seed: `CreateRandom(key_hash, uniqueIDForThisGame, days_played)`
/// = xxHash32(LE pack [key_hash, uid, days_played, 0, 0]). Uses the full uid.
pub fn synced_random_seed(key_hash: i32, uid: u64, days_played: i32) -> i32 {
    let kh = (key_hash as f64 % 2147483647.0) as i32;
    let u = (uid as f64 % 2147483647.0) as i32;
    let dp = (days_played as f64 % 2147483647.0) as i32;
    let mut buf = [0u8; 20];
    buf[0..4].copy_from_slice(&kh.to_le_bytes());
    buf[4..8].copy_from_slice(&u.to_le_bytes());
    buf[8..12].copy_from_slice(&dp.to_le_bytes());
    xxh32(&buf, 0) as i32
}
