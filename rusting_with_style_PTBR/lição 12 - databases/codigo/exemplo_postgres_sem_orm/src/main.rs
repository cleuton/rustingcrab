use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    // 1. Conectar ao banco de dados de forma síncrona
    //    Ajuste a string de conexão conforme seu ambiente (host, user, password, dbname).
    let mut cliente = Client::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )?;

    // 2. Criar a tabela "pessoas" se ela não existir
    //    Aqui, criamos duas colunas: id (chave primária) e nome (VARCHAR).
    cliente.execute(
        "CREATE TABLE IF NOT EXISTS pessoas (
            id SERIAL PRIMARY KEY,
            nome VARCHAR NOT NULL
        )",
        &[],
    )?;

    // 3. Inserir dados na tabela
    //    Usamos placeholders ($1) para evitar SQL injection.
    cliente.execute(
        "INSERT INTO pessoas (nome) VALUES ($1)",
        &[&"João da Silva"],
    )?;

    // 4. Consultar dados da tabela
    //    A função query retorna um vetor de linhas (Row).
    let linhas_encontradas = cliente.query("SELECT id, nome FROM pessoas", &[])?;

    // 5. Exibir os resultados obtidos
    for linha in linhas_encontradas {
        // "linha.get(0)" retorna o valor da primeira coluna (id), do tipo i32
        let identificador: i32 = linha.get(0);
        // "linha.get(1)" retorna o valor da segunda coluna (nome), do tipo String
        let nome_da_pessoa: String = linha.get(1);

        println!("ID: {}, Nome: {}", identificador, nome_da_pessoa);
    }

    // 6. Retornar Ok(()) indicando sucesso
    Ok(())
}