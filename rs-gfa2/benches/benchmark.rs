// TODO: USE ME SENPAI (and improve me if possible :*)

use criterion::*;

use std::path::PathBuf;
use glob::glob;

use gfa2::{
    parser_gfa::parse_gfa,
    parser_gfa2::parse_gfa as parse_gfa2,
};

fn bench_files() {
    for e in glob("./test/gfas/gfa1_files/*").expect("Failed to read directory") {
        let filename = e.unwrap().display().to_string();
        println!("Bench file {}...", filename);
        parse_gfa(&PathBuf::from(filename));        
    }

    for e in glob("./test/gfas/gfa2_files/*").expect("Failed to read directory") {
        let filename = e.unwrap().display().to_string();
        println!("Bench file {}...", filename);
        parse_gfa2(&PathBuf::from(filename));        
    }
}

fn benchmark(c: &mut Criterion) {
    let mut bench_group = c.benchmark_group("auto-benchmark");
    bench_group.sampling_mode(SamplingMode::Auto); 
    bench_group.bench_function("Bench all test files", |f| f.iter(|| bench_files()));
    bench_group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
