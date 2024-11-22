use criterion::{criterion_group, criterion_main, Criterion};
use petgraph::visit::EdgeRef;
use alt_pow::graph_generator::GraphConfig;
use alt_pow::edmonds_karp_solver::EdmondsKarp;

const DENSITY:f64 = 0.25;
const RAND_SEED:u64 = 42;

fn bench_1000(c: &mut Criterion) {
    let config = GraphConfig::new(1_000, (1, 100), RAND_SEED, DENSITY);
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    c.bench_function("Edmonds-Karp: 1,000 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

fn bench_1500(c: &mut Criterion) {
    let config = GraphConfig::new(1_500, (1, 100), RAND_SEED, DENSITY);
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    c.bench_function("Edmonds-Karp: 1,500 vertices", |b| {
        b.iter(|| {
            let mut ek = EdmondsKarp::new();
            for edge in graph.edge_references() {
                ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
            }
            let max_flow = ek.max_flow(source, sink);
            criterion::black_box(max_flow); // Prevent compiler optimizations
        });
    });
}

/*
fn bench_verifier(c: &mut Criterion) {
    let config = GraphConfig::new(1_000, (1, 100), RAND_SEED, DENSITY);
    let graph = config.create_random_flow_graph();
    let source = 0;
    let sink = 999;

    let mut ek = EdmondsKarp::new();
    for edge in graph.edge_references() {
        ek.add_edge(edge.source().index(), edge.target().index(), *edge.weight() as i32);
    }
    let max_flow = ek.max_flow(source, sink);

    c.bench_function("Verifier: 1,000 vertices", |b| {
        b.iter(|| {
            let result = verify_max_flow(&ek, source, sink, max_flow);
            criterion::black_box(result); // Prevent compiler optimizations
        });
    });
}
*/


fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(20)
}

criterion_group! {
    name = benches;
    config = custom_criterion();
    targets = bench_1000, bench_1500 //, bench_verifier
}
criterion_main!(benches);
