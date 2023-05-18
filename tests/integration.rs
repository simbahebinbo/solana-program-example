#![cfg(feature = "test-sbf")]

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};

#[test]
fn test_validator_transaction() {
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("program_template", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("get_latest_blockhash");

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![AccountMeta::new_readonly(sysvar::rent::id(), false)],
            data: vec![1, 2, 3],
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
}
