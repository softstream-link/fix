use criterion::{black_box, criterion_group, criterion_main, Criterion};

use fix_model_core::{prelude::*, unittest::setup};
use log::info;
const K_BYTE: usize = 1 * 1024;
const N_10BYTE_CHUNKS: usize = K_BYTE / 10;
// const MBYTE: usize = KBYTE * 1024;

fn message_str_write(c: &mut Criterion) {
    setup::log::configure();
    let mut msg = Serializer::with_capacity(K_BYTE);

    info!("msg: '{}'", msg);
    c.bench_function("message_str_write", |b| {
        b.iter(|| {
            black_box({
                msg.write_str(1000, "67890");
                msg.clear();
            })
        })
    });
}

// fn message_str_read_rev(c: &mut Criterion) {
//     setup::log::configure();
//     let mut msg = Message::with_capacity(K_BYTE);
//     msg.write_str(1000, "67890");
//     for _ in 0..N_10BYTE_CHUNKS - 1 {
//         msg.write_slice(b"\x01234567890");
//     }
//     info!("msg: '{}'", msg);
//     let str = msg.read_str_rev(1000).unwrap();
//     let str = msg.read_str_rev1(1000).unwrap();
//     info!("str: '{}'", str.as_ref());
//     c.bench_function("message_str_read_rev", |b| {
//         b.iter(|| {
//             black_box({
//                 let _str = msg.read_str_rev(1000).unwrap();
//                 // let str = msg.read_str_rev1(1000).unwrap();
//                 // let _s= str.as_ref();
//             })
//         })
//     });
// }
fn message_str_read_fwd(c: &mut Criterion) {
    setup::log::configure();
    let mut msg = Serializer::with_capacity(K_BYTE);
    msg.write_str(1000, "67890");
    for _ in 0..N_10BYTE_CHUNKS - 1 {
        msg.write_slice(b"\x01234567890");
    }
    info!("msg: '{}'", msg);
    let str = msg.read_str_fwd(1000).unwrap();
    info!("str: '{}'", str);
    c.bench_function("message_str_read_fwd", |b| {
        b.iter(|| {
            black_box({
                let _str = msg.read_str_fwd(1000).unwrap();
            })
        })
    });
}

criterion_group!(benches, message_str_write,  message_str_read_fwd);

criterion_main!(benches);
