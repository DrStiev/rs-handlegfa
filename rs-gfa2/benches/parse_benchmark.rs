use criterion::{
    black_box, 
    criterion_group, 
    criterion_main, 
    Criterion, 
};

use std::path::PathBuf;

// TODO: improve the performance of the parser with big files

fn bench_big_file(c: &mut Criterion){
    c.bench_function("A-3105.sort.gfa test", |f| f.iter(|| parse_gfa(&PathBuf::from("test\\gfas\\big_file\\A-3105.sort.gfa"))));
}
criterion_group!(benches, bench_big_file);
criterion_main!(benches);