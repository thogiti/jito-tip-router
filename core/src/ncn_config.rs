// Global configuration for the tip router

// Contains:
// Main NCN address - updatable?
// Admins
// - config admin - should this be here? or just use main NCN admin?
// - Weight table upload admin (hot wallet) (this exists in NCN, do we want it here too? since custom weight table)
// - Tie breaker admin (hot wallet) (depending on tie breaker process?)
// DAO fee share
// NCN fee share

use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{discriminators::Discriminators, fees::Fees};

#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct NcnConfig {
    /// The Restaking program's NCN admin is the signer to create and update this account
    pub ncn: Pubkey,

    pub tie_breaker_admin: Pubkey,

    pub fee_admin: Pubkey,

    pub fees: Fees,

    /// Bump seed for the PDA
    pub bump: u8,

    /// Reserved space
    reserved: [u8; 127],
}

impl Discriminator for NcnConfig {
    const DISCRIMINATOR: u8 = Discriminators::Config as u8;
}

impl NcnConfig {
    pub const fn new(
        ncn: Pubkey,
        tie_breaker_admin: Pubkey,
        fee_admin: Pubkey,
        fees: Fees,
    ) -> Self {
        Self {
            ncn,
            tie_breaker_admin,
            fee_admin,
            fees,
            bump: 0,
            reserved: [0; 127],
        }
    }

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"config".to_vec()]
    }

    pub fn find_program_address(program_id: &Pubkey, ncn: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = vec![b"config".to_vec(), ncn.to_bytes().to_vec()];
        let (address, bump) = Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            program_id,
        );
        (address, bump, seeds)
    }

    /// Loads the NCN [`Config`] account
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `ncn` - The NCN pubkey
    /// * `account` - The account to load
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    pub fn load(
        program_id: &Pubkey,
        ncn: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, ncn).0)
        {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
