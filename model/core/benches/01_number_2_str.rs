use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::io::Write;

fn number_2_str_write(c: &mut Criterion) {
    c.bench_function("number_2_str_write", |b| {
        b.iter(|| {
            black_box({
                const MAX_U32_TAG_LEN: usize = 10;
                let mut buf = [b'\0'; MAX_U32_TAG_LEN];
                write!(&mut buf[..], "{}", u32::MAX).expect("failed to write to buffer");
                let end = buf.iter().position(|v| *v == b'\0').unwrap_or(MAX_U32_TAG_LEN);
                let _slice = &buf[..end];
            })
        })
    });
}

fn number_2_str_format(c: &mut Criterion) {
    c.bench_function("number_2_str_format", |b| {
        b.iter(|| {
            black_box({
                let tag = format!("{}", u32::MAX);
                let _slice = tag.as_bytes();
            })
        })
    });
}

criterion_group!(benches, number_2_str_write, number_2_str_format);

criterion_main!(benches);
