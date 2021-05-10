use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(
    count: i32,
) -> anyhow::Result<(
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
    (&'static str, i32),
)> {
    cmp_polymorphism::do_enum_obj(count)
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    match process_one(criterion::black_box(criterion::black_box(10))) {
        Ok(((ct, cn), (dt, dn), (ut, un), (rt, rn), (gt, gn))) => {
            assert_eq!(ct, "Meow!");
            assert_eq!(cn, 10);
            assert_eq!(dt, "Woof!");
            assert_eq!(dn, 10 + 10 + 1);
            assert_eq!(ut, "Quack!");
            assert_eq!(un, 10 + 10 + 1 + 10 + 2);
            assert_eq!(rt, "Caw!");
            assert_eq!(rn, 10 + 10 + 1 + 10 + 2 + 10 + 3);
            assert_eq!(gt, "Croak!");
            assert_eq!(gn, 10 + 10 + 1 + 10 + 2 + 10 + 3 + 10 + 4);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("enum_obj^", |b| {
        b.iter(|| {
            let _r = process_one(criterion::black_box(10));
        })
    });
    //
    {
        let vec = cmp_polymorphism::create_enum_objs(criterion::black_box(1000));
        c.bench_function("enum_obj^vec^id^01k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_id_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^sum^01k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_sum_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^rem^01k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_rem_enum_objs(&vec);
            })
        });
    }
    //
    {
        let vec = cmp_polymorphism::create_enum_objs(criterion::black_box(8000));
        c.bench_function("enum_obj^vec^id^08k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_id_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^sum^08k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_sum_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^rem^08k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_rem_enum_objs(&vec);
            })
        });
    }
    //
    {
        let vec = cmp_polymorphism::create_enum_objs(criterion::black_box(90000));
        c.bench_function("enum_obj^vec^id^90k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_id_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^sum^90k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_sum_enum_objs(&vec);
            })
        });
        c.bench_function("enum_obj^vec^rem^90k", |b| {
            b.iter(|| {
                let _r = cmp_polymorphism::sum_rem_enum_objs(&vec);
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte)
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1500));
    targets = criterion_benchmark);
criterion_main!(benches);
