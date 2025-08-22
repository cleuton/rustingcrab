use regex::Regex;
use std::collections::BTreeMap;

/// Função propositalmente lenta:
/// - Compila Regex em cada linha (caro).
/// - Compila outro Regex por palavra.
/// - Aloca/copia demais (`to_string`, `to_lowercase`, `format!`, `clone`).
/// - Recopia/ordena o mapa no final.
pub fn contar_palavras_lento(texto: &str) -> BTreeMap<String, usize> {
    let mut resultado = BTreeMap::new();

    for linha in texto.lines() {
        let re = Regex::new(r"[A-Za-zÀ-ÖØ-öø-ÿ0-9_]+").unwrap();

        for m in re.find_iter(linha) {
            let mut w = m.as_str().to_string();

            let re2 = Regex::new(
                r"(^[^A-Za-z0-9À-ÖØ-öø-ÿ_]+|[^A-Za-z0-9À-ÖØ-öø-ÿ_]+$)"
            ).unwrap();
            w = re2.replace_all(&w, "").to_string();

            let w = w.to_lowercase();
            let count = resultado.get(&w).cloned().unwrap_or(0);
            resultado.insert(format!("{}", w), count + 1);
        }
    }

    let mut v: Vec<_> = resultado.iter().collect();
    v.sort_by_key(|(k, _)| k.len());

    let mut resultado2 = BTreeMap::new();
    for (k, v) in v {
        resultado2.insert(k.clone(), *v);
    }
    resultado2
}

pub fn soma_tamanhos(mapa: &BTreeMap<String, usize>) -> usize {
    mapa.iter().map(|(k, v)| k.len() + v).sum()
}
