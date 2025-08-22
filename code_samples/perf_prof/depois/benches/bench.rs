use criterion::{criterion_group, criterion_main, Criterion};
use slowwords::{contar_palavras_rapido, soma_tamanhos};
use std::fs::File;
use std::hint::black_box;
use std::time::Duration;

fn texto_grande() -> String {
    // Carga pequena para o benchmark (rápido) e suficiente para o profiler em loop.
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

    // Benchmark curto (sem precisar passar flags)
    c.bench_function("contar_palavras_rapido", |b| {
        b.iter(|| {
            let mapa = contar_palavras_rapido(black_box(&dados));
            black_box(soma_tamanhos(&mapa))
        })
    });

    // ---- Perfil focado com pprof ----
    // Se a execução for muito curta, não há amostras -> SVG vazio.
    // Por isso, repetimos a função sob o guard para acumular amostras.
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000) // 1000 Hz: amostragem alta
        .build()
        .expect("iniciar pprof");

    // Repete a função lenta várias vezes só durante o profiling (rápido o bastante).
    // Ajuste N se quiser mais/menos densidade.
    let dados_prof = &dados;
    let mut acc = 0usize;
    let n = 2000;
    for _ in 0..n {
        let mapa = contar_palavras_rapido(dados_prof);
        acc ^= black_box(soma_tamanhos(&mapa));
    }
    black_box(acc);

    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").expect("criar flamegraph.svg");
        report.flamegraph(file).expect("escrever flamegraph.svg");
        eprintln!("OK: flamegraph.svg gerado na raiz do projeto");
    } else {
        eprintln!("Aviso: pprof não coletou amostras suficientes. Aumente `n` ou `frequency`.");
    }
}

// Config curta por padrão (sem flags na CLI)
criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    targets = bench_contagem
}
criterion_main!(benches);
