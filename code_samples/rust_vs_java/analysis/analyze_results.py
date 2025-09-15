#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Analisa resultados do benchmark (Rust vs Java) e gera results/comparison.json.

- Pode ser executado a partir de QUALQUER diretório.
- Lê:
    results/<server>_k6_results.csv
    results/<server>_resource_usage.csv  (ou fallback para <server>_monitoring.csv)
    results/<server>_binary_size.txt
- Escreve:
    results/comparison.json

Requer: pandas
"""

import json
from statistics import mean, pstdev
from pathlib import Path
from typing import Optional, List

import pandas as pd

# Raiz do projeto: .../performance-test
ROOT = Path(__file__).resolve().parents[1]
RESULTS_DIR = ROOT / "results"

# Servidores a comparar (ajuste aqui se mudar nomes)
SERVERS: List[str] = ["rust_axum", "java_undertow"]


def rp(*parts) -> Path:
    """Path dentro de results/"""
    return RESULTS_DIR.joinpath(*parts)


def read_csv_safe(path: Path, **kwargs) -> Optional[pd.DataFrame]:
    if not path.exists():
        print(f"[WARN] CSV não encontrado: {path}")
        return None
    try:
        return pd.read_csv(path, **kwargs)
    except Exception as e:
        print(f"[WARN] Falha ao ler CSV {path}: {e}")
        return None


def resolve_monitoring_csv(server_name: str) -> Optional[Path]:
    """
    Tenta localizar o CSV de monitoramento por nomes conhecidos.
    Ordem de preferência:
      1) <server>_resource_usage.csv
      2) <server>_monitoring.csv
    """
    candidates = [
        rp(f"{server_name}_resource_usage.csv"),
        rp(f"{server_name}_monitoring.csv"),
    ]
    for p in candidates:
        if p.exists():
            if p.name.endswith("_monitoring.csv"):
                print(f"[INFO] Usando arquivo de monitoramento (fallback): {p}")
            return p
    # Se nenhum existir, retorne o primeiro (para mensagem consistente em read_csv_safe)
    return candidates[0]


def analyze_k6(server_name: str):
    """
    Lê results/<server>_k6_results.csv e extrai estatísticas de http_req_duration
    e taxa de sucesso via 'checks'.

    Suporta variações de colunas entre versões do k6:
      - 'metric_name' ou 'metric'
      - 'metric_value' ou 'value'
    """
    path = rp(f"{server_name}_k6_results.csv")
    df = read_csv_safe(path)
    if df is None or df.empty:
        return None

    # normalizar nomes possíveis
    cols = {c.lower(): c for c in df.columns}
    c_metric = cols.get("metric_name") or cols.get("metric")
    c_value = cols.get("metric_value") or cols.get("value")

    if not c_metric or not c_value:
        print(f"[WARN] Colunas esperadas não encontradas em {path} (metric_name/value).")
        return None

    # durations
    dur = df[df[c_metric].astype(str) == "http_req_duration"]
    durations = pd.to_numeric(dur[c_value], errors="coerce").dropna().tolist()

    # checks = 1 sucesso, 0 falha
    chk = df[df[c_metric].astype(str) == "checks"]
    checks = pd.to_numeric(chk[c_value], errors="coerce").dropna().tolist()

    if len(durations) == 0:
        print(f"[WARN] Nenhuma duração válida em {path}")
        return None

    return {
        "min_response_time": float(min(durations)),
        "max_response_time": float(max(durations)),
        "avg_response_time": float(mean(durations)),
        "std_response_time": float(pstdev(durations)) if len(durations) > 1 else 0.0,
        "total_requests": int(len(durations)),
        "success_rate": float(100.0 * (sum(checks) / len(checks))) if len(checks) > 0 else float("nan"),
    }


def analyze_resources(server_name: str):
    """
    Lê results/<server>_resource_usage.csv (ou <server>_monitoring.csv) e extrai estatísticas de CPU e Memória.
    Tenta detectar colunas por nomes comuns:
      - CPU: cpu, cpu_percent, cpu_usage, ...
      - Mem: memory, memory_mb, mem_mb, rss_mb, ...
    """
    path = resolve_monitoring_csv(server_name)
    df = read_csv_safe(path)
    if df is None or df.empty:
        return None

    lc = {c.lower(): c for c in df.columns}
    c_cpu = lc.get("cpu") or lc.get("cpu_percent") or lc.get("cpu_usage")
    c_mem = lc.get("memory") or lc.get("memory_mb") or lc.get("mem_mb") or lc.get("rss_mb")

    if not c_cpu or not c_mem:
        # fallback: primeira coluna que contenha cpu / mem|rss
        cand_cpu = [c for c in df.columns if "cpu" in c.lower()]
        cand_mem = [c for c in df.columns if ("mem" in c.lower() or "rss" in c.lower() or "memory" in c.lower())]
        c_cpu = c_cpu or (cand_cpu[0] if cand_cpu else None)
        c_mem = c_mem or (cand_mem[0] if cand_mem else None)

    if not c_cpu or not c_mem:
        print(f"[WARN] Colunas de CPU/Memória não detectadas em {path}.")
        return None

    cpu_vals = pd.to_numeric(df[c_cpu], errors="coerce").dropna().tolist()
    mem_vals = pd.to_numeric(df[c_mem], errors="coerce").dropna().tolist()

    if len(cpu_vals) == 0 or len(mem_vals) == 0:
        print(f"[WARN] Amostras insuficientes em {path}.")
        return None

    def smin(v): return float(min(v))
    def smax(v): return float(max(v))
    def savg(v): return float(mean(v))
    def sstd(v): return float(pstdev(v)) if len(v) > 1 else 0.0

    return {
        "min_cpu": smin(cpu_vals),
        "max_cpu": smax(cpu_vals),
        "avg_cpu": savg(cpu_vals),
        "std_cpu": sstd(cpu_vals),
        "min_memory": smin(mem_vals),
        "max_memory": smax(mem_vals),
        "avg_memory": savg(mem_vals),
        "std_memory": sstd(mem_vals),
    }


def read_binary_size(server_name: str):
    """
    Lê results/<server>_binary_size.txt com o tamanho do artefato:
      - Rust: binário
      - Java: JAR 'all' (Maven Shade)
    """
    path = rp(f"{server_name}_binary_size.txt")
    if not path.exists():
        print(f"[WARN] Tamanho do artefato não encontrado: {path}")
        return None
    try:
        return int(path.read_text(encoding="utf-8").strip())
    except Exception as e:
        print(f"[WARN] Falha ao ler tamanho do artefato de {server_name}: {e}")
        return None


def main():
    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    results = {}

    for server in SERVERS:
        print(f"[INFO] Analisando {server}...")
        k6_stats = analyze_k6(server)
        resource_stats = analyze_resources(server)
        binary_size = read_binary_size(server)

        results[server] = {
            "k6": k6_stats,
            "resources": resource_stats,
            "binary_size": binary_size,
        }

    out_path = rp("comparison.json")
    with out_path.open("w", encoding="utf-8") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)

    print(f"[OK] Análise concluída. Arquivo salvo em {out_path}")


if __name__ == "__main__":
    main()
