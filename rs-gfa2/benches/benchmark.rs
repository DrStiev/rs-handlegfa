use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

use bstr::io::*;
use bstr::BString;

use gfa2::{
    gfa2::*,
    tag::*,
    parser_gfa2::*,
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn load_lines(path: &PathBuf) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).byte_lines();
    let result = lines.map(|l| l.unwrap()).collect();
    Ok(result)
}

fn parse_lines<T: OptFields>(input: &[Vec<u8>]) -> GFA2<BString, T> {
    let parser: GFA2Parser<BString, T> = GFA2Parser::new();
    parser.parse_lines(input.iter()).unwrap()
}

fn parse_lines_noopt(input: &[Vec<u8>]) -> GFA2<BString, ()> {
    parse_lines(input)
}

fn parse_lines_withopt(input: &[Vec<u8>]) -> GFA2<BString, OptionalFields> {
    parse_lines(input)
}

fn parse_lines_usize<T: OptFields>(input: &[Vec<u8>]) -> GFA2<usize, T> {
    let parser: GFA2Parser<usize, T> = GFA2Parser::new();
    parser.parse_lines(input.iter()).unwrap()
}

fn parse_lines_usize_noopt(input: &[Vec<u8>]) -> GFA2<usize, ()> {
    parse_lines_usize(input)
}

fn parse_lines_usize_withopt(input: &[Vec<u8>]) -> GFA2<usize, OptionalFields> {
    parse_lines_usize(input)
}

static GFAPATH: &str = "./test/gfa2_files/";

macro_rules! bench_gfa {
    ($parser:ident, $id:literal, $name:ident, $gfa:literal) => {
        fn $name(c: &mut Criterion) {
            let mut path = PathBuf::from(GFAPATH);
            path.push($gfa);
            let lines: Vec<Vec<u8>> = load_lines(&path).unwrap();
            c.bench_with_input(BenchmarkId::new($id, $gfa), &lines, |b, l| {
                b.iter(|| $parser(&l));
            });
        }
    };
}

macro_rules! bench_gfa_noopt {
    ($name:ident, $gfa:literal) => {
        bench_gfa!(parse_lines_usize_noopt, "excluding_optionals", $name, $gfa);
    };
}

macro_rules! bench_gfa_withopt {
    ($name:ident, $gfa:literal) => {
        bench_gfa!(
            parse_lines_usize_withopt,
            "including_optionals",
            $name,
            $gfa
        );
    };
}

// bench_gfa_noopt!(cov_noopt, "relabeledSeqs.gfa");
bench_gfa_noopt!(big_noopt, "big.gfa");
bench_gfa_noopt!(graph_nicernames_noopt, "graph_nicernames.gfa");
bench_gfa_noopt!(irl_noopt, "irl.gfa");
bench_gfa_noopt!(sample_noopt, "sample.gfa");
bench_gfa_noopt!(sample2_noopt, "sample2.gfa");

// bench_gfa_withopt!(cov_withopt, "relabeledSeqs.gfa");
bench_gfa_withopt!(big_withopt, "big.gfa");
bench_gfa_withopt!(graph_nicernames_withopt, "graph_nicernames.gfa");
bench_gfa_withopt!(irl_withopt, "irl.gfa");
bench_gfa_withopt!(sample_withopt, "sample.gfa");
bench_gfa_withopt!(sample2_withopt, "sample2.gfa");

criterion_group!(
    name = no_opt_benches;
    config = Criterion::default().sample_size(25);
    targets = /*cov_noopt,*/ big_noopt, graph_nicernames_noopt, irl_noopt, sample_noopt, sample2_noopt
);

criterion_group!(
    name = with_opt_benches;
    config = Criterion::default().sample_size(25);
    targets = /*cov_withopt,*/ big_withopt, graph_nicernames_withopt, irl_withopt, sample_withopt, sample2_withopt
);

criterion_main!(no_opt_benches, with_opt_benches);

