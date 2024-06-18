use ahash::RandomState;
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

static RANDOM_STATE: Lazy<RandomState> = Lazy::new(RandomState::new);

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hash_ip_ahash(ip: &IpAddr) -> u32 {
    RANDOM_STATE.hash_one(ip) as u32
}

pub fn hash_str_ahash(ip: &str) -> u32 {
    RANDOM_STATE.hash_one(ip) as u32
}

pub fn hash_ip_md5(ip: &IpAddr) -> u32 {
    match ip {
        IpAddr::V4(ip) => u32::from_ne_bytes(md5::compute(&ip.octets())[..4].try_into().unwrap()),
        IpAddr::V6(ip) => u32::from_ne_bytes(md5::compute(&ip.octets())[..4].try_into().unwrap()),
    }
}
pub fn hash_str_md5(ip: &str) -> u32 {
    u32::from_ne_bytes(md5::compute(ip)[..4].try_into().unwrap())
}

pub fn hash_ip_sha256(ip: &IpAddr) -> u32 {
    let mut hasher = Sha256::new();
    match ip {
        IpAddr::V4(ip) => hasher.update(&ip.octets()),
        IpAddr::V6(ip) => hasher.update(&ip.octets()),
    }
    u32::from_ne_bytes(hasher.finalize()[..4].try_into().unwrap())
}
pub fn hash_str_sha256(ip: &str) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(ip);
    u32::from_ne_bytes(hasher.finalize()[..4].try_into().unwrap())
}

pub fn random_ip() -> IpAddr {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    if rng.gen() {
        IpAddr::V4(Ipv4Addr::from(rng.gen::<u32>()))
    } else {
        IpAddr::V6(Ipv6Addr::from(rng.gen::<u128>()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
