use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, instruction::AccountMeta,
    program::invoke, pubkey, pubkey::Pubkey, sysvar::Sysvar,
};

solana_program::entrypoint!(process_instruction);

const RAYDIUM_AMM_V4_PROGRAM: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

// 0.1.0 版本的问题是用时间戳有±1s的误差范围太大 改用slot粒度更细(后记:其实slot不好 到底是confirm的slot还是process的slot模糊不清)
pub fn process_instruction(
    _program_id: &Pubkey,     // FfukznxHiszK53PbQwsr9LLM7wzwUq8F7oT2AudkteLv
    accounts: &[AccountInfo], // [AccountInfo; 20]
    input_data: &[u8],        // [u8; 26] 8+1+17(raydium)
) -> ProgramResult {
    let slot_when_req = u64::from_le_bytes(input_data[0..8].try_into().unwrap());
    let expire_in_slots = u64::from(input_data[8]);

    let clock = Clock::get()?;
    let slot_diff = clock.slot.saturating_sub(slot_when_req);
    if slot_diff > expire_in_slots {
        // 因为是预言机的时间戳会滞后，可能会比传入的时间戳小1 减出来得到负1
        // solana_program::msg!("{}-{}", current_timestamp, timestamp);
        return Err(solana_program::program_error::ProgramError::Custom(
            slot_diff as u32,
        ));
    }
    let remaining_data = &input_data[9..];

    // 最后两个账户是当前合约地址、下个要CPI调用合约的地址
    // solana 的安全机制 客户端请求必须列出所有 CPI 调用路径上的智能合约地址
    //   防止恶意合约调用客户端请求AccountInfo以外的合约
    //   也禁止智能合约AccountInfo::new()凭空创建客户端没传入的地址
    // 有个任意CPI调用的文档 https://solana.com/developers/courses/program-security/arbitrary-cpi
    let len = accounts.len();
    let account_metas = accounts
        .iter()
        .take(len - 2) // remove curr_program_addr and raydium_program_addr(cpi required)
        .map(|a| AccountMeta {
            pubkey: *a.key,
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect::<Vec<_>>();
    let ix = solana_program::instruction::Instruction {
        program_id: RAYDIUM_AMM_V4_PROGRAM,
        accounts: account_metas,
        data: remaining_data.to_vec(),
    };
    invoke(&ix, accounts)?;

    Ok(())
}

