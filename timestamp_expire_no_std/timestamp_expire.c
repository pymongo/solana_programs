#include <solana_sdk.h>
uint64_t sol_get_clock_sysvar(uint8_t *ret);
// https://github.com/solana-labs/solana-program-library/blob/master/examples/c/makefile
extern uint64_t entrypoint(const uint8_t *input) {
    // input += 2*sizeof(uint64_t); // skip len of accounts/data
    int64_t req_timestamp = 0;
    req_timestamp |= (int64_t)input[16+0];
    req_timestamp |= (int64_t)input[16+1] << 8;
    req_timestamp |= (int64_t)input[16+2] << 16;
    req_timestamp |= (int64_t)input[16+3] << 24;
    int64_t expire_in = (int64_t)input[4];

    uint8_t clock[40]; // [u64; 5], last is timestmap
    uint64_t r = sol_get_clock_sysvar(clock);
    if (r != SUCCESS) {
        return r;
    }
    int64_t timestamp = (int64_t)clock[0] |
        ((int64_t)clock[32+1] << 8) |
        ((int64_t)clock[32+2] << 16) |
        ((int64_t)clock[32+3] << 24) |
        ((int64_t)clock[32+4] << 32) |
        ((int64_t)clock[32+5] << 40) |
        ((int64_t)clock[32+6] << 48) |
        ((int64_t)clock[32+7] << 56);
    if (timestamp <= req_timestamp) {
        return SUCCESS;
    }
    int64_t duration = timestamp - req_timestamp;
    if (duration > expire_in) {
        return (uint64_t)duration;
    }
    return SUCCESS;
}
/* C的版本不能用 系统时间的预言机 莫名其妙 时间戳越用越旧
// var expireInProgram = solana.MustPublicKeyFromBase58("A376icvBm1BjTxtvRBrQXppEKY5kacp2npW42fN17MhN")
22:29:30.558462 arb_check.go:156: S LUCE Gnx8i5MznsLoxhmPsuun59JZFhKodk4yyE8eDXtNFBUgWcxTYLadDaoT4utF9kM9iyFN2Hau4LyVV3Z2irVGeBU map[InstructionError:[2 map[Custom:37]]] 305990380
22:29:32.398635 arb_check.go:156: S LUCE MNJFsRDqzrhGedEAkL62MydJRBW94j5YJ61VRVKPWNsbEfufHgingXcketa2Z8nuabcmEeK15rAaojY8g4peSMe map[InstructionError:[2 map[Custom:39]]] 305990387
22:29:33.242109 arb_check.go:156: S LUCE 2XsZnVEcKH2mTvhNtBZkLMD7mXiLqoEvGvbJieWHPZRar9GThQnVb52XbTVwPAGgKkMKGhruksF79wmMVHC2qbuB map[InstructionError:[2 map[Custom:39]]] 305990389
*/