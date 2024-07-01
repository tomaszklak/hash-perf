use std::net::IpAddr;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_perf::{
    hash_ip_ahash, hash_ip_blake2s256, hash_ip_blake3, hash_ip_md5, hash_ip_sha256,
    hash_ip_std_hash, hash_str_ahash, hash_str_blake2s256, hash_str_blake3, hash_str_md5,
    hash_str_sha256, hash_str_std_hash, random_ip,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let ips: Vec<IpAddr> = (0..1000).map(|_| random_ip()).collect();

    c.bench_function("ahash", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_ahash(black_box(ip));
            }
        })
    });
    c.bench_function("std", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_std_hash(black_box(ip));
            }
        })
    });
    c.bench_function("blake3", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_blake3(black_box(ip));
            }
        })
    });
    c.bench_function("md5", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_md5(black_box(ip));
            }
        })
    });
    c.bench_function("blake2s256", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_blake2s256(black_box(ip));
            }
        })
    });

    c.bench_function("sha256", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_ip_sha256(black_box(ip));
            }
        })
    });

    let ips: Vec<String> = ips.into_iter().map(|ip| ip.to_string()).collect();

    c.bench_function("str_ahash", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_ahash(black_box(ip));
            }
        })
    });

    c.bench_function("str_std", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_std_hash(black_box(ip));
            }
        })
    });
    c.bench_function("str_blake3", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_blake3(black_box(ip));
            }
        })
    });
    c.bench_function("str_md5", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_md5(black_box(ip));
            }
        })
    });
    c.bench_function("str_blake2s256", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_blake2s256(black_box(ip));
            }
        })
    });
    c.bench_function("str_sha256", |b| {
        b.iter(|| {
            for ip in &ips {
                hash_str_sha256(black_box(ip));
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
