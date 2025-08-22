use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slowwords::{contar_palavras_lento, soma_tamanhos};
use std::fs::File;
use std::time::Duration;

fn texto_grande() -> String {
    // Carga pequena por padrão (pode subir depois se quiser flame maior).
    let vezes: usize = 200;

    let chunk = "Rust rust RUST, desempenho; memória? segurança! \
                 regex expressões EXPRESSOES; dados dados dados. \
                 Palavra palavra PALAVRA número123 número123 número123.\n";
    let mut s = String::with_capacity(vezes * chunk.len());
    for _ in 0..vezes {
        s.push_str(chunk);
    }
    s
}

fn bench_contagem(c: &mut Criterion) {
    let dados = texto_grande();

    // Benchmark rápido (poucas amostras, tempos curtos)
    c.bench_function("contar_palavras_lento", |b| {
        b.iter(|| {
            let mapa = contar_palavras_lento(black_box(&dados));
            black_box(soma_tamanhos(&mapa))
        })
    });

    // Execução única com pprof para gerar o flamegraph
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(800)
        .build()
        .expect("iniciar pprof");

    let mapa = contar_palavras_lento(&dados);
    let _ = black_box(mapa);

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").expect("criar flamegraph.svg");
        report.flamegraph(file).expect("escrever flamegraph.svg");
        eprintln!("OK: flamegraph.svg gerado na raiz do projeto");
    }
}

// Config padrão já curta, sem precisar passar flags na CLI
criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)                         // mínimo aceitável
        .warm_up_time(Duration::from_secs(1))    // aquece rápido
        .measurement_time(Duration::from_secs(1)); // mede rápido
    targets = bench_contagem
}
criterion_main!(benches);
