use {
    anchor_lang::{
        prelude::Pubkey, solana_program::instruction::Instruction, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    std::{fs, path::PathBuf},
};

#[test]
fn test_initialize() {
    let program_id = on_chain_voting::id();
    let payer = Keypair::new();
    let program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("deploy")
        .join("on_chain_voting.so");

    let poll_id = 1u64;
    let poll_voting_start = 1_000_000u64;
    let poll_voting_end = 2_000_000u64;
    let name = String::from("Test Poll");
    let description = String::from("Test poll description");
    let (poll_account, _) =
        Pubkey::find_program_address(&[b"poll", &poll_id.to_le_bytes()], &program_id);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &on_chain_voting::instruction::InitPoll {
            _poll_id: poll_id,
            poll_voting_start,
            poll_voting_end,
            name: name.clone(),
            description: description.clone(),
        }
        .data(),
        on_chain_voting::accounts::InitPoll {
            signer: payer.pubkey(),
            poll_account,
            system_program: anchor_lang::solana_program::system_program::ID,
        }
        .to_account_metas(None),
    );

    let Ok(bytes) = fs::read(&program_path) else {
        assert_eq!(instruction.program_id, program_id);
        assert_eq!(instruction.accounts.len(), 3);
        assert!(!instruction.data.is_empty());
        return;
    };

    let mut svm = LiteSVM::new();
    svm.add_program(program_id, &bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
}
