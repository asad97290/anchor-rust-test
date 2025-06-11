use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        system_program,
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file, signer::Signer,
    },
    Client, Cluster,
};
use anchor_rust_test::{Counter,accounts,instruction};

#[test]
fn test_initialize() {
    let program_id = "7bP565Kfda36jnWLjeHj5eJc8pMxzLDNWLs8Vvr9Too6";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();


     // Derive the counter PDA (Program Derived Address)
    let (counter_pda, _bump) = Pubkey::find_program_address(
        &[b"counter"],
        &program_id,
    );
    // Initialize the counter account
    program
        .request()
        .accounts(accounts::Initialize {
            counter:counter_pda,
            payer:payer.pubkey(),
            system_program:system_program::ID,
        })
        .args(instruction::Initialize {})
        .send()
        .expect("");

    // Check that the counter account was initialized correctly
    let counter_account: Counter = program.account::<Counter>(counter_pda).unwrap();
    assert_eq!(counter_account.count, 0);
    
   

}
