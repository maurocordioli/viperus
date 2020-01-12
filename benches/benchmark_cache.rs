use criterion::{black_box, criterion_group, criterion_main, Criterion};
use viperus::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    viperus::load_file(&path!(".", "assets", "cache.yaml"), Format::YAML).unwrap();
    viperus::load_file(&path!(".", "assets", "cache.env"), Format::ENV).unwrap();
    viperus::add("level1.key_add", true);



    let mut cnt = 0;
    c.bench_function("glob get bconfig bool key", |b| {
        b.iter(|| {
            let res = viperus::get::<bool>("level1.key_bool").unwrap();
            if res {
                cnt = cnt + 1;
            };
        })
    });
    c.bench_function("glob get override bool key", |b| {
        b.iter(|| {
            let res = viperus::get::<bool>("level1.key_add").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });

    c.bench_function("glob get env bool key", |b| {
        b.iter(|| {
            let res = viperus::get::<bool>("level1.key_env").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });


    let mut v = viperus::Viperus::new();
    v.load_file(&path!(".", "assets", "cache.yaml"), Format::YAML).unwrap();
    v.load_file(&path!(".", "assets", "cache.env"), Format::ENV).unwrap();
    v.add("level1.key_add", true);



    let mut cnt = 0;
    c.bench_function("inst get config bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_bool").unwrap();
            if res {
                cnt = cnt + 1;
            };
        })
    });
    v.cache(true);
    c.bench_function("inst cached get config bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_bool").unwrap();
            if res {
                cnt = cnt + 1;
            };
        })
    });
    v.cache(false);
    

    c.bench_function("inst get override bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_add").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });

    v.cache(true);
    c.bench_function("inst cached get override bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_add").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });
    v.cache(false);

    c.bench_function("inst get env bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_env").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });

    v.cache(true);

    c.bench_function("inst cached get env bool key", |b| {
        b.iter(|| {
            let res = v.get::<bool>("level1.key_env").unwrap();
            if res {
                cnt = cnt + 1;
            }
        })
    });
    v.cache(false);

}


criterion_group!(benches, criterion_benchmark);
 

criterion_main!(benches);
