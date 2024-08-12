use bevy::app::prelude::*;
use criterion::Criterion;

pub fn update(c: &mut Criterion) {
    let mut group = c.benchmark_group("app::app::update");

    group.bench_function("empty", |b| {
        let mut app = App::empty();
        b.iter(|| app.update());
    });

    group.bench_function("new", |b| {
        let mut app = App::new();
        b.iter(|| app.update());
    });
}
