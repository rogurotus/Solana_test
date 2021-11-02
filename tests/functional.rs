#![allow(non_snake_case)]
use test_case::processor::{process_instruction, Myinstruction};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_case() {
    let program_id = Pubkey::new_unique();

    let firstA_pubkey = Pubkey::new_unique();
    let firstB_pubkey = Pubkey::new_unique();
    let secondA_pubkey = Pubkey::new_unique();
    let secondB_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "test_case", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    let mut add_acc = |a| 
    {
        program_test.add_account(
            a,
            Account {
                lamports: 100000,
                data: vec![0_u8; mem::size_of::<u32>()],
                owner: program_id,
                ..Account::default()
            },
        );
    };
    add_acc(firstA_pubkey);
    add_acc(firstB_pubkey);
    add_acc(secondA_pubkey);
    add_acc(secondB_pubkey);

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;


    let FA_account = banks_client
        .get_account(firstA_pubkey)
        .await
        .expect("get_account")
        .expect("FA not found");
    assert_eq!(
        FA_account.lamports,
        100000
    );

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            program_id,
            &Myinstruction::SendA(42),
            vec![
                AccountMeta::new(firstA_pubkey, false),
                AccountMeta::new(firstB_pubkey, false),
                AccountMeta::new(secondA_pubkey, false),
                AccountMeta::new(secondB_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    let FA_account = banks_client
        .get_account(firstA_pubkey)
        .await
        .expect("get_account")
        .expect("FA not found");
    assert_eq!(
        FA_account.lamports,
        99958
    );

    let FB_account = banks_client
        .get_account(firstB_pubkey)
        .await
        .expect("get_account")
        .expect("FA not found");
    assert_eq!(
        FB_account.lamports,
        100042
    );

    let SA_account = banks_client
        .get_account(secondA_pubkey)
        .await
        .expect("get_account")
        .expect("FA not found");
    assert_eq!(
        SA_account.lamports,
        100042
    );

    let SB_account = banks_client
        .get_account(secondB_pubkey)
        .await
        .expect("get_account")
        .expect("FA not found");
    assert_eq!(
        SB_account.lamports,
        99958
    );

}