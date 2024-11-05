use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use shank::{ShankAccount, ShankType};
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{discriminators::Discriminators, error::TipRouterError, jito_number::JitoNumber};

// PDA'd ["WEIGHT_TABLE", NCN, NCN_EPOCH_SLOT]
#[derive(Debug, Clone, Copy, Zeroable, ShankType, Pod, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct WeightTable {
    /// The NCN on-chain program is the signer to create and update this account,
    /// this pushes the responsibility of managing the account to the NCN program.
    ncn: Pubkey,

    /// The NCN epoch for which the weight table is valid
    ncn_epoch: PodU64,

    /// Slot weight table was created
    slot_created: PodU64,

    /// Slot weight table was finalized
    slot_finalized: PodU64,

    /// Bump seed for the PDA
    bump: u8,

    /// Reserved space
    reserved: [u8; 128],

    /// The weight table
    table: [WeightEntry; 32],
}

impl Discriminator for WeightTable {
    const DISCRIMINATOR: u8 = Discriminators::WeightTable as u8;
}

impl WeightTable {
    pub const MAX_TABLE_ENTRIES: usize = 32;
    pub const NOT_FINALIZED: u64 = u64::MAX;

    pub fn new(ncn: Pubkey, ncn_epoch: u64, slot_created: u64, bump: u8) -> Self {
        Self {
            ncn,
            ncn_epoch: PodU64::from(ncn_epoch),
            slot_created: PodU64::from(slot_created),
            slot_finalized: PodU64::from(Self::NOT_FINALIZED),
            bump,
            reserved: [0; 128],
            table: [WeightEntry::default(); Self::MAX_TABLE_ENTRIES],
        }
    }

    pub fn seeds(ncn: &Pubkey, ncn_epoch: u64) -> Vec<Vec<u8>> {
        Vec::from_iter(
            [
                b"WEIGHT_TABLE".to_vec(),
                ncn.to_bytes().to_vec(),
                ncn_epoch.to_le_bytes().to_vec(),
            ]
            .iter()
            .cloned(),
        )
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        ncn_epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, ncn_epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    pub fn get_mints(&self) -> Vec<Pubkey> {
        self.table
            .iter()
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.mint)
            .collect()
    }

    pub fn get_mint_hash(mints: Vec<Pubkey>) -> u64 {
        let mut hash = 0;

        // Makes sure the hash is the same regardless of the order of the mints
        let mut sorted_mints = mints;
        sorted_mints.sort();

        for mint in sorted_mints {
            let bytes = mint.to_bytes();
            let u64_slice = u64::from_le_bytes(bytes[0..8].try_into().unwrap());

            hash ^= u64_slice;
        }

        hash
    }

    pub fn check_mints_okay(&self, mint_hash: u64, mint_count: u8) -> Result<(), TipRouterError> {
        if mint_count != self.entry_count() as u8 {
            return Err(TipRouterError::WeightMintsDoNotMatchLength);
        }

        let table_mint_hash = Self::get_mint_hash(self.get_mints());
        if mint_hash != table_mint_hash {
            return Err(TipRouterError::WeightMintsDoNotMatchMintHash);
        }

        Ok(())
    }

    pub fn entry_count(&self) -> usize {
        self.table.iter().filter(|entry| !entry.is_empty()).count()
    }

    pub fn find_weight(&self, mint: &Pubkey) -> Option<JitoNumber> {
        self.table
            .iter()
            .find(|entry| entry.mint == *mint)
            .map(|entry| entry.weight)
    }

    pub fn set_weight(&mut self, mint: &Pubkey, weight: JitoNumber) -> Result<(), TipRouterError> {
        // First, try to find an existing entry with the given mint
        if let Some(entry) = self.table.iter_mut().find(|entry| entry.mint == *mint) {
            entry.weight = weight;
            return Ok(());
        }

        // If no existing entry found, look for the first empty slot
        if let Some(entry) = self.table.iter_mut().find(|entry| entry.is_empty()) {
            entry.mint = *mint;
            entry.weight = weight;
            return Ok(());
        }

        // If no existing entry and no empty slots, return error
        Err(TipRouterError::NoMoreTableSlots)
    }

    pub const fn ncn(&self) -> Pubkey {
        self.ncn
    }

    pub fn ncn_epoch(&self) -> u64 {
        self.ncn_epoch.into()
    }

    pub fn slot_created(&self) -> u64 {
        self.slot_created.into()
    }

    pub fn slot_finalized(&self) -> u64 {
        self.slot_finalized.into()
    }

    pub fn finalized(&self) -> bool {
        self.slot_finalized != PodU64::from(Self::NOT_FINALIZED)
    }

    pub fn finalize(&mut self, current_slot: u64) {
        self.slot_finalized = PodU64::from(current_slot);
    }

    pub fn load(
        program_id: &Pubkey,
        weight_table: &AccountInfo,
        ncn: &AccountInfo,
        ncn_epoch: u64,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if weight_table.owner.ne(program_id) {
            msg!("Weight table account is not owned by the program");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if weight_table.data_is_empty() {
            msg!("Weight table account is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !weight_table.is_writable {
            msg!("Weight table account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if weight_table.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Weight table account has an incorrect discriminator");
            return Err(ProgramError::InvalidAccountData);
        }
        let expected_pubkey = Self::find_program_address(program_id, ncn.key, ncn_epoch).0;
        if weight_table.key.ne(&expected_pubkey) {
            msg!("Weight table incorrect PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy, Zeroable, ShankType, Pod)]
#[repr(C)]
pub struct WeightEntry {
    pub mint: Pubkey,
    pub weight: JitoNumber,
}

impl WeightEntry {
    pub const fn new(mint: Pubkey, weight: JitoNumber) -> Self {
        Self { weight, mint }
    }

    pub fn is_empty(&self) -> bool {
        self.mint.eq(&Pubkey::default())
    }
}

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use super::*;

    #[test]
    fn test_weight_table_new() {
        let ncn = Pubkey::new_unique();
        let table = WeightTable::new(ncn, 0, 0, 0);
        assert_eq!(table.entry_count(), 0);
    }

    #[test]
    fn test_weight_table_finalize() {
        let mut weight_table = WeightTable::new(Pubkey::new_unique(), 0, 0, 0);

        assert!(!weight_table.finalized());
        assert_eq!(weight_table.slot_finalized(), WeightTable::NOT_FINALIZED);

        weight_table.finalize(0);
        assert!(weight_table.finalized());
    }
}
