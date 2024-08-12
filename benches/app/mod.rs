mod app;

use criterion::criterion_group;

criterion_group!(group, app::update,);
