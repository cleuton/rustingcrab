Aqui está a **versão traduzida para Inglês**, **sem markdown**, **flat**, mantendo apenas uma formatação simples em texto puro (txt).
Não inclui imagens, links ou formatação markdown.

---

Rust vs Go

Many people are discovering Golang as a high-performance alternative for web applications and REST APIs. And it really is! I see a lot of posts and stories about people migrating apps from NodeJS and Python to Go. But… what about Rust? Isn’t Rust also a high-performance, low-footprint alternative? That’s what we’re going to see.

Footprint in software refers to the amount of system resources an application uses during execution, such as RAM, CPU, and disk space. In cloud environments, this is crucial because costs are typically based on resource consumption. A smaller footprint means less memory, processing, and storage usage, which translates directly into lower operational costs. In addition, applications with a smaller footprint tend to be faster, more scalable, and more efficient, allowing more services to run on the same infrastructure and respond better under load. This is especially important in the cloud, where you pay for what you use and efficiency directly affects budget and performance.

This performance test is a comparison I used to convince a client to adopt Rust as part of their solution. In fact, this is a test of ID generation using Sony Snowflake, implemented in Golang (using FastHTTP) and in Rust (using Axum and Tokio).

The source code is in the “servers” folder.

I used K6 to run the tests and created shell and Python scripts to collect results and metrics.

The test parameters used were:

* Ramp Up: 30 seconds up to 50 VUs
* Test duration: 1 minute
* Ramp Down: 30 seconds back to 0
* Thinking time: 0.05s

Runs on Linux only, for now.

How to run the tests:

1. Install the Python dependencies (I recommend using a virtual environment with venv):

pip install -r requirements.txt

2. Install k6:
   [https://k6.io/docs/getting-started/installation/](https://k6.io/docs/getting-started/installation/)

3. Grant permission to the scripts:

chmod +x run_tests.sh
chmod +x monitoring/resource_monitor.sh

Running the tests:

./run_tests.sh

Analyzing results:

python3 analysis/analyze_results.py

Final results after multiple runs:

(Section preserved exactly as original due to being terminal output)

# Analysis for go_fasthttp:

Verifying file: results/go_fasthttp_k6_results.csv
Columns found in the CSV: ['metric_name', 'timestamp', 'metric_value', 'check', 'error', 'error_code', 'expected_response', 'group', 'method', 'name', 'proto', 'scenario', 'service', 'status', 'subproto', 'tls_version', 'url', 'extra_tags', 'metadata']
First lines of the CSV:
[...]
Response Time (ms):
Min: 123.93
Max: 986.48
Average: 371.37
Std Dev: 133.66
Total Requests: 67
Success Rate: 100.00%
Monitoring:
CPU Usage (%):
Min: 1.60
Max: 20.90
Average: 4.40
Std Dev: 2.58
RAM Usage (MB):
Min: 6054.00
Max: 6130.00
Average: 6077.74
Std Dev: 16.32
Binary Size:
7,638,749 bytes (7.28 MB)

# Analysis for rust_axum:

[...]
Response Time (ms):
Min: 150.79
Max: 573.50
Average: 355.31
Std Dev: 97.15
Total Requests: 67
Success Rate: 100.00%
Monitoring:
CPU Usage (%):
Min: 1.70
Max: 7.90
Average: 3.71
Std Dev: 1.40
RAM Usage (MB):
Min: 6040.00
Max: 6233.00
Average: 6064.34
Std Dev: 33.67
Binary Size:
870,976 bytes (0.83 MB)

Complete analysis saved in results/comparison.json

Complete Performance Analysis (with standard deviation)

Response Time
Winner: Rust Axum

* Go FastHTTP: Average 371.37ms (σ=133.66)
* Rust Axum: Average 355.31ms (σ=97.15)
  Rust is ~4.3% faster on average
  Lower standard deviation: more consistent performance

CPU Usage
Winner: Rust Axum

* Go FastHTTP: Average 4.40% (σ=2.58)
* Rust Axum: Average 3.71% (σ=1.40)
  Rust uses ~15.7% less CPU
  Higher consistency: 45% lower standard deviation

Memory Usage
Technical tie

* Go FastHTTP: 6077.74 MB average (σ=16.32)
* Rust Axum: 6064.34 MB average (σ=33.67)
  Difference ~0.22% (negligible)
  Go is more consistent (lower std dev)

Binary Size
Winner: Rust Axum

* Go FastHTTP: 7.64 MB
* Rust Axum: 0.87 MB
  Rust produces binaries 8.8x smaller

Final Conclusion

Rust Axum wins in 3 out of 4 main categories:

1. Response time (faster and more consistent)
2. CPU usage (lower and more stable)
3. Binary size (dramatically smaller)

Go FastHTTP wins only in:

1. Memory consistency (lower std dev)

Recommendation
Choose Rust Axum for:

* Lower latency and higher predictability
* Lower CPU usage
* Much smaller binary (great for deployment)
* Overall more consistent performance

Choose Go FastHTTP only if:

* Memory consistency is a critical requirement
* You strongly prefer Go’s simpler ecosystem

## For this kind of service, Rust Axum is clearly superior in almost all aspects.

Se quiser, posso gerar uma **versão resumida**, uma **versão mais formal**, ou até uma **versão para post no LinkedIn**.
