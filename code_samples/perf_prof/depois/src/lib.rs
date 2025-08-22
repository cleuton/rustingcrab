use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeMap;

// Compila uma única vez, em tempo de execução, thread-safe.
static RE_WORD: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[A-Za-zÀ-ÖØ-öø-ÿ0-9_]+").unwrap());

/// Versão otimizada: não recompila regex e evita cópias inúteis.
/// Mantém a mesma assinatura/retorno do exemplo anterior.
pub fn contar_palavras_rapido(texto: &str) -> BTreeMap<String, usize> {
    // Usa BTreeMap para manter ordenação por chave (compatível com o seu retorno).
    let mut resultado = BTreeMap::<String, usize>::new();

    for linha in texto.lines() {
        // Reaproveita o RE_WORD estático já compilado.
        for m in RE_WORD.find_iter(linha) {
            // `to_lowercase` gera String; usamos entry para evitar buscas duplas.
            let w = m.as_str().to_lowercase();
            *resultado.entry(w).or_insert(0) += 1;
        }
    }

    // Se for necessário “ordenar por tamanho da palavra” como antes,
    // faça isso fora daqui. Evitei a recópia final para reduzir trabalho.
    resultado
}

/// Soma tamanhos (mesma utilitária do exemplo anterior).
pub fn soma_tamanhos(mapa: &BTreeMap<String, usize>) -> usize {
    mapa.iter().map(|(k, v)| k.len() + v).sum()
}
