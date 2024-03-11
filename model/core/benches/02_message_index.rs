use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fix_model_core::{prelude::*, unittest::setup};

fn message_index(c: &mut Criterion) {
    setup::log::configure();
    let mut msg = setup::model::get_large_fix_message();

    c.bench_function("message_index", |b| {
        b.iter(|| {
            black_box({
                let res = msg.index();
                assert!(res.is_ok());
            })
        })
    });
}

fn message_read(c: &mut Criterion) {
    setup::log::configure();
    let mut msg = setup::model::get_large_fix_message();
    let res = msg.index();
    assert!(res.is_ok());
    c.bench_function("message_read", |b| {
        b.iter(|| {
            black_box({
                let res = msg.read_str_1(100);
                assert!(res.is_some());
            })
        })
    });
}

criterion_group!(benches, message_index, message_read);

criterion_main!(benches);
