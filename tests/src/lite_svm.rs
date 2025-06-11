use {
    anchor_client::anchor_lang::solana_program::example_mocks::solana_sdk::system_program,
    litesvm::LiteSVM,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::Keypair,
    solana_message::{
        v0::Message as MessageV0, AddressLookupTableAccount, Message, VersionedMessage,
    },
    solana_pubkey::{pubkey, Pubkey},
    solana_signer::Signer,
    solana_transaction::{versioned::VersionedTransaction, Transaction},
};

#[test]
fn test_lite_svm() {
    let from_keypair = Keypair::new();
    let from = from_keypair.pubkey();

    let mut svm = LiteSVM::new();
    let program_id = pubkey!("7bP565Kfda36jnWLjeHj5eJc8pMxzLDNWLs8Vvr9Too6");

    let bytes = include_bytes!("../../target/deploy/anchor_rust_test.so");
    svm.add_program(program_id, bytes);
    let (counter_pda, _bump) = Pubkey::find_program_address(&[b"counter"], &program_id);

    svm.airdrop(&from, 1000000000).unwrap();

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(counter_pda, false),
                AccountMeta::new(from, true),
                AccountMeta::new(system_program::ID, false),
            ],
            data: [175, 175, 109, 31, 13, 152, 155, 237].to_vec(), // initialize discriminator
        }],
        Some(&from),
        &blockhash,
    );
    let tx = Transaction::new(&[from_keypair.insecure_clone()], msg, blockhash);
    svm.send_transaction(tx).unwrap();

    let account_data = svm.get_account(&counter_pda).unwrap().data;
    let value_bytes = &account_data[8..];

    assert_eq!(value_bytes, 0u64.to_le_bytes().to_vec());

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new(counter_pda, false)],
            data: [11, 18, 104, 9, 104, 174, 59, 33].to_vec(), // increment discriminator
        }],
        Some(&from),
        &blockhash,
    );
    let tx = Transaction::new(&[from_keypair], msg, blockhash);
    svm.send_transaction(tx).unwrap();




      let account_data = svm.get_account(&counter_pda).unwrap().data;
    let value_bytes = &account_data[8..];

    assert_eq!(value_bytes, 1u64.to_le_bytes().to_vec());
}
