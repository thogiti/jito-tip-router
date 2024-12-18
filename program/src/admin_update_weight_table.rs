use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_signer, load_token_mint};
use jito_restaking_core::ncn::Ncn;
use jito_tip_router_core::{error::TipRouterError, weight_table::WeightTable};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

/// Updates weight table
pub fn process_admin_update_weight_table(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    ncn_epoch: u64,
    weight: u128,
) -> ProgramResult {
    let [ncn, weight_table, weight_table_admin, mint, restaking_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Ncn::load(restaking_program.key, ncn, false)?;
    let ncn_weight_table_admin = {
        let ncn_data = ncn.data.borrow();
        let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;
        ncn.weight_table_admin
    };

    load_signer(weight_table_admin, true)?;
    load_token_mint(mint)?;
    WeightTable::load(program_id, weight_table, ncn, ncn_epoch, true)?;

    if restaking_program.key.ne(&jito_restaking_program::id()) {
        msg!("Incorrect restaking program ID");
        return Err(ProgramError::InvalidAccountData);
    }

    if ncn_weight_table_admin.ne(weight_table_admin.key) {
        msg!("Vault update delegations ticket is not at the correct PDA");
        return Err(TipRouterError::IncorrectWeightTableAdmin.into());
    }

    let mut weight_table_data = weight_table.try_borrow_mut_data()?;
    let weight_table_account = WeightTable::try_from_slice_unchecked_mut(&mut weight_table_data)?;

    weight_table_account.check_initialized()?;

    weight_table_account.set_weight(mint.key, weight, Clock::get()?.slot)?;

    Ok(())
}
