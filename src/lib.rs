use solana_program::{account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(&format!(
        "process_instruction: {}: {} accounts, data={:?} with spl-token {}",
        program_id,
        accounts.len(),
        instruction_data,
        spl_token::id(),
    ));
    Ok(())
}

#[cfg(test)]
mod test {
    use assert_matches::*;
    use solana_program::instruction::{AccountMeta, Instruction};
    use solana_program_test::*;
    use solana_sdk::{signature::Signer, transaction::Transaction};

    use super::*;

    #[tokio::test]
    async fn test_transaction() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) =
            ProgramTest::new("program_template", program_id, processor!(process_instruction))
                .start()
                .await;

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![AccountMeta::new(payer.pubkey(), false)],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);

        assert_matches!(
            banks_client.process_transaction(transaction).await,
            Ok(())
        );
    }
}
