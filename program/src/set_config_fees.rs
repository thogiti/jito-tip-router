use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::loader::{load_signer, load_system_account};
use jito_restaking_core::ncn::Ncn;
use jito_tip_router_core::ncn_config::NcnConfig;
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult,
    program_error::ProgramError, pubkey::Pubkey, sysvar::Sysvar,
};

pub fn process_set_config_fees(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    new_dao_fee_bps: Option<u64>,
    new_ncn_fee_bps: Option<u64>,
    new_block_engine_fee_bps: Option<u64>,
    new_fee_wallet: Option<Pubkey>,
) -> ProgramResult {
    let [config, ncn_account, ncn_admin, restaking_program_id] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_system_account(config, true)?;
    load_signer(ncn_admin, true)?;

    NcnConfig::load(program_id, config, true)?;
    Ncn::load(restaking_program_id.key, ncn_account, false)?;

    let mut config_data = config.try_borrow_mut_data()?;
    if config_data[0] != NcnConfig::DISCRIMINATOR {
        return Err(ProgramError::InvalidAccountData);
    }
    let config = NcnConfig::try_from_slice_unchecked_mut(&mut config_data)?;

    // Verify NCN and Admin
    if config.ncn != *ncn_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    let ncn_data = ncn_account.data.borrow();
    let ncn = Ncn::try_from_slice_unchecked(&ncn_data)?;
    if ncn.admin != *ncn_admin.key {
        return Err(ProgramError::InvalidArgument);
    }

    let epoch = Clock::get()?.epoch;
    config.fees.set_new_fees(
        new_dao_fee_bps,
        new_ncn_fee_bps,
        new_block_engine_fee_bps,
        new_fee_wallet,
        epoch,
    )?;

    Ok(())
}