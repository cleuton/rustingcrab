use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use toml::{map::Map, Value};

#[derive(Debug)]
pub struct Regra {
    pub tamanho_senha: i32,
    pub maiusculas: bool,
    pub minusculas: bool,
    pub caracteres_especiais: bool,
    pub pelo_menos_um_digito: bool,
    pub pelo_menos_um_caracter_especial: bool,
    pub pelo_menos_uma_letra_maiuscula: bool,
}

impl Regra {
    // Função para construir uma instância de Regra a partir de um Value do TOML
    pub fn from_toml(value: &Value) -> Result<Self, io::Error> {
        let tabela = value.as_table().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Tabela 'regra' inválida ou ausente")
        })?;

        Ok(Self {
            tamanho_senha: tabela.get("tamanho_senha")
                .and_then(|v| v.as_integer())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'tamanho_senha' inválido ou ausente"))? as i32,
            maiusculas: tabela.get("maiusculas")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'maiusculas' inválido ou ausente"))?,
            minusculas: tabela.get("minusculas")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'minusculas' inválido ou ausente"))?,
            caracteres_especiais: tabela.get("caracteres_especiais")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'caracteres_especiais' inválido ou ausente"))?,
            pelo_menos_um_digito: tabela.get("pelo_menos_um_digito")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_um_digito' inválido ou ausente"))?,
            pelo_menos_um_caracter_especial: tabela.get("pelo_menos_um_caracter_especial")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_um_caracter_especial' inválido ou ausente"))?,
            pelo_menos_uma_letra_maiuscula: tabela.get("pelo_menos_uma_letra_maiuscula")
                .and_then(|v| v.as_bool())
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Campo 'pelo_menos_uma_letra_maiuscula' inválido ou ausente"))?,
        })
    }
}

pub fn salvar_senha(url: &str, senha: &str) -> io::Result<()> {
    let caminho = "config.toml";
    let conteudo = fs::read_to_string(caminho)?;
    let mut config: Value = conteudo.parse::<Value>()?;

    // Pega a tabela raiz do arquivo TOML
    let config_table = config.as_table_mut().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Formato inválido no arquivo TOML")
    })?;

    // Verifica se a tabela [senhas] existe
    if let Some(tabela) = config_table.get_mut("senhas") {
        // A tabela já existe, atualiza com os valores padrão
        if let Some(tabela_map) = tabela.as_table_mut() {
            tabela_map.insert(url.to_string(), Value::String(senha.to_string()));
        }
    } else {
        // A tabela não existe, cria uma nova tabela com os valores padrão
        let mut valores_padrao = Map::new();
        valores_padrao.insert(url.to_string(), Value::String(senha.to_string()));
        config_table.insert("senhas".to_string(), Value::Table(valores_padrao));
    }

    // Escreve o arquivo atualizado
    let conteudo_atualizado = toml::to_string(&config).expect("Erro ao converter para TOML");
    let mut arquivo = File::create(caminho)?;
    arquivo.write_all(conteudo_atualizado.as_bytes())?;

    Ok(())

}

pub fn ler_regra() -> io::Result<Regra> {
    let caminho = "config.toml";

    // 1. Verificar se o arquivo existe
    if !Path::new(caminho).exists() {
        println!("Arquivo config não encontrado. Criando com valores padrão...");

        let regra_def = Regra {
            tamanho_senha: 12,
            maiusculas: true,
            minusculas: true,
            caracteres_especiais: true,
            pelo_menos_um_digito: true,
            pelo_menos_um_caracter_especial: true,
            pelo_menos_uma_letra_maiuscula: true,
        };

        // Criar valores padrão para o arquivo TOML
        let mut valores_padrao = Map::new();
        let mut regra = Map::new();
        regra.insert("tamanho_senha".to_string(), Value::Integer(regra_def.tamanho_senha.into()));
        regra.insert("maiusculas".to_string(), Value::Boolean(regra_def.maiusculas));
        regra.insert("minusculas".to_string(), Value::Boolean(regra_def.minusculas));
        regra.insert("caracteres_especiais".to_string(), Value::Boolean(regra_def.caracteres_especiais));
        regra.insert("pelo_menos_um_digito".to_string(), Value::Boolean(regra_def.pelo_menos_um_digito));
        regra.insert("pelo_menos_um_caracter_especial".to_string(), Value::Boolean(regra_def.pelo_menos_um_caracter_especial));
        regra.insert("pelo_menos_uma_letra_maiuscula".to_string(), Value::Boolean(regra_def.pelo_menos_uma_letra_maiuscula));
        valores_padrao.insert("regra".to_string(), Value::Table(regra));

        // Converter para string TOML
        let conteudo_padrao = toml::to_string(&Value::Table(valores_padrao))
            .expect("Erro ao converter valores padrão para TOML");

        // Criar o arquivo e escrever os valores padrão
        let mut arquivo = File::create(caminho)?;
        arquivo.write_all(conteudo_padrao.as_bytes())?;

        println!("Arquivo config criado com sucesso!");
        return Ok(regra_def);
    }

    // 2. Ler o arquivo TOML existente
    let conteudo = fs::read_to_string(caminho)?;
    let config: Value = conteudo.parse::<Value>().expect("Erro ao analisar o TOML");

    // 3. Extrair a tabela [regra] e convertê-la para a struct Regra
    let regra = config.get("regra")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Tabela 'regra' não encontrada"))?;

    Regra::from_toml(regra)
}


