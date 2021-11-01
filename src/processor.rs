#![allow(non_snake_case)]


use std::ops::Deref;

use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Myinstruction
{
    SendA(u64),
    SendB(u64),
}

pub struct UserAccount<'a,'b>
{
    TokenA: &'a AccountInfo<'b>,
    TokenB: &'a AccountInfo<'b>,
}
impl<'a,'b> UserAccount<'a,'b>
{
    fn new<T: Iterator<Item = &'a AccountInfo<'b>>>(mut iter: &mut T) -> Result<Self, ProgramError>
    {
        let TokenA = next_account_info(&mut iter)?;
        let TokenB = next_account_info(&mut iter)?;
        Ok(UserAccount
        {
            TokenA,
            TokenB,
        })
    }

    fn SendATokens(&mut self, to_acc: &mut UserAccount, mut dx: u64) -> ProgramResult
    {

        let X = to_acc.TokenA.lamports.as_ref().borrow().to_owned();
        let Y = to_acc.TokenB.lamports.as_ref().borrow().to_owned();
        let K = X*Y;
        let mut dy = Y - K / (X + dx);


        let check_A_balance = self.TokenA.lamports.as_ref().borrow().deref().ge(&&mut dx);
        let check_B_balance = to_acc.TokenB.lamports.as_ref().borrow().deref().ge(&&mut dy);

        if !check_A_balance || !check_B_balance
        {
            return Err(ProgramError::AccountNotRentExempt);
        }
        
        **self.TokenA.lamports.borrow_mut() -= dx;
        **to_acc.TokenA.lamports.borrow_mut()+= dx;

        
        **to_acc.TokenB.lamports.borrow_mut() -= dy;
        **self.TokenB.lamports.borrow_mut() += dy;

        Ok(())
    }

    fn SendBTokens(&mut self, to_acc: &mut UserAccount, mut dy: u64) -> ProgramResult
    {

        let X = to_acc.TokenA.lamports.as_ref().borrow().to_owned();
        let Y = to_acc.TokenB.lamports.as_ref().borrow().to_owned();
        let K = X*Y;
        let mut dx = X - K / (Y + dy);


        let check_A_balance = to_acc.TokenA.lamports.as_ref().borrow().deref().ge(&&mut dx);
        let check_B_balance = self.TokenB.lamports.as_ref().borrow().deref().ge(&&mut dy);

        if !check_A_balance || !check_B_balance
        {
            return Err(ProgramError::AccountNotRentExempt);
        }
        
        **self.TokenB.lamports.borrow_mut() -= dy;
        **to_acc.TokenB.lamports.borrow_mut()+= dy;

        
        **to_acc.TokenA.lamports.borrow_mut() -= dx;
        **self.TokenA.lamports.borrow_mut() += dx;

        Ok(())
    }
}



pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
    ) -> ProgramResult 
{
    let mut accounts_iter = accounts.iter();

    let mut from_acc = UserAccount::new(&mut accounts_iter)?;
    let mut to_acc = UserAccount::new(&mut accounts_iter)?;

    if !accounts.iter().all(|acc| acc.owner == program_id) {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    match Myinstruction::try_from_slice(_instruction_data)? 
    {
        Myinstruction::SendA(tokens) => {from_acc.SendATokens(&mut to_acc, tokens)?},
        Myinstruction::SendB(tokens) => {from_acc.SendBTokens(&mut to_acc, tokens)?}
    }

    Ok(())
}