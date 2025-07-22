fn main() {
    tonic_build::compile_protos("proto/tarefa.proto")
        .expect("Falha ao compilar proto/tarefa.proto");
}
