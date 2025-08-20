use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};

#[test]
#[ignore] // execute manualmente quando quiser chamar a devnet
fn invoca_na_devnet() {
    // usa a chave padrão do solana-cli (~/.config/solana/id.json)
    let payer_path = dirs::home_dir().unwrap().join(".config/solana/id.json");
    let payer: Keypair = read_keypair_file(payer_path).expect("carregar keypair");

    let rpc = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    // seu program id
    let program_id =
        Pubkey::from_str("<O PROGRAM_ID ENTRA AQUI>").unwrap();

    // sua instrução não usa contas nem dados
    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: vec![],
    };

    let blockhash = rpc.get_latest_blockhash().expect("pegar blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );

    let sig = rpc
        .send_and_confirm_transaction(&tx)
        .expect("enviar transação");
    eprintln!("Tx sig: {}", sig);
}
