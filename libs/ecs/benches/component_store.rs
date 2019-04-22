use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ecs::{Component, ComponentStore};

#[derive(Debug)]
struct ComponentZST;
impl Component for ComponentZST {}

#[derive(Debug, Eq, PartialEq)]
struct ComponentWord(usize);
impl Component for ComponentWord {}

#[derive(Debug, Eq, PartialEq)]
struct ComponentStr(&'static str);
impl Component for ComponentStr {}

fn component_store(c: &mut Criterion) {
    c.bench_function("get ZST (present)", |b| {
        let mut cs = ComponentStore::new();
        for _ in 0..5000 {
            cs.new_entity();
        }
        let e = cs.new_entity();
        for _ in 0..5000 {
            cs.new_entity();
        }
        cs.set_component(e, ComponentZST);
        b.iter(|| cs.get_component::<ComponentZST>(e))
    });

    c.bench_function("get ZST (not present)", |b| {
        let mut cs = ComponentStore::new();
        for _ in 0..5000 {
            cs.new_entity();
        }
        let e = cs.new_entity();
        for _ in 0..5000 {
            cs.new_entity();
        }
        b.iter(|| cs.get_component::<ComponentZST>(e))
    });

    c.bench_function("get word-sized (present)", |b| {
        let mut cs = ComponentStore::new();
        for _ in 0..5000 {
            cs.new_entity();
        }
        let e = cs.new_entity();
        for _ in 0..5000 {
            cs.new_entity();
        }
        cs.set_component(e, ComponentWord(12345));
        b.iter(|| cs.get_component::<ComponentWord>(e))
    });

    c.bench_function("get word-sized (not present)", |b| {
        let mut cs = ComponentStore::new();
        for _ in 0..5000 {
            cs.new_entity();
        }
        let e = cs.new_entity();
        for _ in 0..5000 {
            cs.new_entity();
        }
        b.iter(|| {
            black_box(cs.get_component::<ComponentWord>(e));
        })
    });

    c.bench_function(
        "get word-sized with null pointer optimization (present)",
        |b| {
            let mut cs = ComponentStore::new();
            for _ in 0..5000 {
                cs.new_entity();
            }
            let e = cs.new_entity();
            for _ in 0..5000 {
                cs.new_entity();
            }
            cs.set_component(e, ComponentStr("12345"));
            b.iter(|| cs.get_component::<ComponentStr>(e))
        },
    );

    c.bench_function(
        "get word-sized with null pointer optimization (not present)",
        |b| {
            let mut cs = ComponentStore::new();
            for _ in 0..5000 {
                cs.new_entity();
            }
            let e = cs.new_entity();
            for _ in 0..5000 {
                cs.new_entity();
            }
            b.iter(|| cs.get_component::<ComponentStr>(e))
        },
    );

    c.bench_function("get not-present, set, get present; word-sized", |b| {
        let mut cs = ComponentStore::new();
        b.iter(|| {
            let e = cs.new_entity();
            assert_eq!(cs.get_component::<ComponentWord>(e), None);
            cs.set_component::<ComponentWord>(e, ComponentWord(12345));
            assert_eq!(
                cs.get_component::<ComponentWord>(e),
                Some(&ComponentWord(12345))
            );
        })
    });
}

criterion_group!(benches, component_store);
criterion_main!(benches);