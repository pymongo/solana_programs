use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, pubkey::Pubkey,
    sysvar::Sysvar,
};

// 38siDDuRXAYDzon7fbUhkJbREv7Pk2RgAKwjAnwdkhMT
solana_program::entrypoint!(process_instruction);

// 0.1.0 版本的问题是用时间戳有±1s的误差范围太大 改用slot粒度更细
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo], // [AccountInfo; 0]
    input_data: &[u8],         // [u8; 5]
) -> ProgramResult {
    let req_timestamp = i64::from(u32::from_le_bytes(input_data[0..4].try_into().unwrap()));
    let expire_in_secound = i64::from(input_data[4]);

    let clock = Clock::get()?;
    let diff = clock.unix_timestamp.saturating_sub(req_timestamp);
    if diff > expire_in_secound {
        return Err(solana_program::program_error::ProgramError::Custom(
            diff as u32,
        ));
    }
    Ok(())
}
