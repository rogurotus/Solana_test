#![allow(non_snake_case)]
use test_case::processor::*;

use borsh::BorshSerialize;

use solana_program::clock::Epoch;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use std::mem;

#[cfg(test)]
#[test]
fn test_sendA() {
    let program_id = Pubkey::default();
    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp1 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp2 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp3 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp4 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let instruction_data: Vec<u8> = Myinstruction::SendA(42).try_to_vec().unwrap();

    let accounts = vec![temp1, temp2, temp3, temp4];

    process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    assert_eq!(accounts[0].lamports.as_ref().borrow().to_owned(), 99958);
    assert_eq!(accounts[1].lamports.as_ref().borrow().to_owned(), 100042);
    assert_eq!(accounts[2].lamports.as_ref().borrow().to_owned(), 100042);
    assert_eq!(accounts[3].lamports.as_ref().borrow().to_owned(), 99958);
}

#[test]
fn test_sendB() {
    let program_id = Pubkey::default();
    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp1 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp2 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp3 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let key = Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![0; mem::size_of::<u32>()];
    let owner = Pubkey::default();
    let temp4 = AccountInfo::new(
        &key,
        false,
        true,
        &mut lamports,
        &mut data,
        &owner,
        false,
        Epoch::default(),
    );

    let instruction_data: Vec<u8> = Myinstruction::SendB(42).try_to_vec().unwrap();

    let accounts = vec![temp1, temp2, temp3, temp4];

    process_instruction(&program_id, &accounts, &instruction_data).unwrap();
    assert_eq!(accounts[0].lamports.as_ref().borrow().to_owned(), 100042);
    assert_eq!(accounts[1].lamports.as_ref().borrow().to_owned(), 99958);
    assert_eq!(accounts[2].lamports.as_ref().borrow().to_owned(), 99958);
    assert_eq!(accounts[3].lamports.as_ref().borrow().to_owned(), 100042);
}
