#![no_std] // optional

use solana_nostd_entrypoint::{
    basic_panic_impl, entrypoint_nostd, noalloc_allocator,
    solana_program::{
        self, clock::Clock, entrypoint::ProgramResult, pubkey::Pubkey, sysvar::Sysvar,
    },
    NoStdAccountInfo,
};

// 6m9RVsDdv9dmytjDXAZjVdXhD95HAe2cScaXtJ18RFFR
entrypoint_nostd!(process_instruction, 32);

noalloc_allocator!();
basic_panic_impl!();

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[NoStdAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let timestamp: u32 = unsafe {
        let ptr = instruction_data[0..4].as_ptr() as *const u32;
        ptr.read()
    };
    let req_timestamp = i64::from(timestamp);
    let expire_in_secound = i64::from(instruction_data[4]);

    let clock = Clock::get()?;
    let diff = clock.unix_timestamp.saturating_sub(req_timestamp);
    if diff > expire_in_secound {
        return Err(solana_program::program_error::ProgramError::Custom(
            diff as u32,
        ));
    }
    Ok(())
}

// #[test]
// fn test_timestamp_parse() {
//     let instruction_data = 1733549621_i64.to_le_bytes();
//     let timestamp: u32 = unsafe {
//         let ptr = instruction_data[0..4].as_ptr() as *const u32;
//         ptr.read()
//     };
//     dbg!(timestamp);
// }
