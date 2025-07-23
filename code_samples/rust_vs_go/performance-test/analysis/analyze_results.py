#!/usr/bin/env python3

import pandas as pd
import numpy as np
import os
import json

def analyze_k6_results(filename):
    """Analisa resultados do K6"""
    if not os.path.exists(filename):
        print(f"Arquivo não encontrado: {filename}")
        return None
    
    try:
        # Ler apenas algumas linhas para debug
        df = pd.read_csv(filename, nrows=1000)  # Limitar para debug
        print(f"Colunas encontradas no CSV: {list(df.columns)}")
        print(f"Primeiras linhas do CSV:")
        print(df.head())
        
        if df.empty:
            print("Arquivo CSV está vazio")
            return None
        
        # Filtrar métricas de http_req_duration
        duration_rows = df[df['metric_name'] == 'http_req_duration']
        
        if len(duration_rows) == 0:
            print("Nenhuma métrica de http_req_duration encontrada")
            return None
        
        # Usar metric_value em vez de value
        durations = duration_rows['metric_value'].dropna()
        
        if len(durations) == 0:
            print("Nenhuma duração válida encontrada")
            return None
        
        # Calcular success rate
        failed_rows = df[df['metric_name'] == 'http_req_failed']
        success_rate = None
        if len(failed_rows) > 0:
            failed_values = failed_rows['metric_value'].dropna()
            if len(failed_values) > 0:
                success_rate = ((failed_values == 0.0).mean() * 100)
        
        return {
            'min_response_time': durations.min() * 1000,  # ms
            'max_response_time': durations.max() * 1000,  # ms
            'avg_response_time': durations.mean() * 1000,  # ms
            'std_response_time': durations.std() * 1000,  # ms
            'total_requests': len(durations),
            'success_rate': success_rate
        }
    except Exception as e:
        print(f"Erro ao analisar arquivo K6: {e}")
        return None

def analyze_resource_usage(filename):
    """Analisa uso de recursos"""
    if not os.path.exists(filename):
        print(f"Arquivo de monitoramento não encontrado: {filename}")
        return None
    
    try:
        df = pd.read_csv(filename)
        
        if df.empty:
            print("Arquivo de monitoramento está vazio")
            return None
            
        return {
            'min_cpu': float(df['cpu_percent'].min()),
            'max_cpu': float(df['cpu_percent'].max()),
            'avg_cpu': float(df['cpu_percent'].mean()),
            'std_cpu': float(df['cpu_percent'].std()),
            'min_memory': float(df['memory_mb'].min()),
            'max_memory': float(df['memory_mb'].max()),
            'avg_memory': float(df['memory_mb'].mean()),
            'std_memory': float(df['memory_mb'].std())
        }
    except Exception as e:
        print(f"Erro ao analisar arquivo de recursos: {e}")
        return None

def get_binary_size(filename):
    """Obtém tamanho do binário"""
    if not os.path.exists(filename):
        print(f"Arquivo de tamanho não encontrado: {filename}")
        return 0
    try:
        with open(filename, 'r') as f:
            return int(f.read().strip())
    except Exception as e:
        print(f"Erro ao ler tamanho do binário: {e}")
        return 0

def main():
    servers = ['go_fasthttp', 'rust_axum']
    results = {}
    
    for server in servers:
        print(f"\nAnálise para {server}:")
        print("=" * 50)
        
        # Análise K6
        k6_file = f"results/{server}_k6_results.csv"
        print(f"Verificando arquivo: {k6_file}")
        k6_stats = analyze_k6_results(k6_file)
        
        if k6_stats:
            print("Tempo de Resposta (ms):")
            print(f"   Min: {k6_stats['min_response_time']:.2f}")
            print(f"   Max: {k6_stats['max_response_time']:.2f}")
            print(f"   Média: {k6_stats['avg_response_time']:.2f}")
            print(f"   Desvio: {k6_stats['std_response_time']:.2f}")
            print(f"   Total Requests: {k6_stats['total_requests']}")
            if k6_stats['success_rate'] is not None:
                print(f"   Success Rate: {k6_stats['success_rate']:.2f}%")
        else:
            print("Nenhum dado K6 disponível")
        
        # Análise de Recursos
        monitor_file = f"results/{server}_monitoring.csv"
        print(f"Verificando arquivo de monitoramento: {monitor_file}")
        resource_stats = analyze_resource_usage(monitor_file)
        
        if resource_stats:
            print("\nUso de CPU (%):")
            print(f"   Min: {resource_stats['min_cpu']:.2f}")
            print(f"   Max: {resource_stats['max_cpu']:.2f}")
            print(f"   Média: {resource_stats['avg_cpu']:.2f}")
            print(f"   Desvio: {resource_stats['std_cpu']:.2f}")
            
            print("\nUso de RAM (MB):")
            print(f"   Min: {resource_stats['min_memory']:.2f}")
            print(f"   Max: {resource_stats['max_memory']:.2f}")
            print(f"   Média: {resource_stats['avg_memory']:.2f}")
            print(f"   Desvio: {resource_stats['std_memory']:.2f}")
        else:
            print("Nenhum dado de recursos disponível")
        
        # Tamanho do binário
        binary_size_file = f"results/{server}_binary_size.txt"
        print(f"Verificando arquivo de tamanho: {binary_size_file}")
        binary_size = get_binary_size(binary_size_file)
        if binary_size > 0:
            print(f"\nTamanho do Executável: {binary_size:,} bytes ({binary_size/1024/1024:.2f} MB)")
        else:
            print("Tamanho do executável não disponível")
        
        results[server] = {
            'k6': k6_stats,
            'resources': resource_stats,
            'binary_size': binary_size
        }
    
    # Salvar resultados em JSON
    with open('results/comparison.json', 'w') as f:
        json.dump(results, f, indent=2)
    
    print("\nAnálise completa! Resultados salvos em results/comparison.json")

if __name__ == "__main__":
    main()