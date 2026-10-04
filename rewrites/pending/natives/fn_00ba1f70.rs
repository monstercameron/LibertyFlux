// original: 0x00BA1F70 SET_DECISION_MAKER_ATTRIBUTE_TEAMWORK
// SET_DECISION_MAKER_ATTRIBUTE_TEAMWORK: forward (decision maker, value).
export!(cdecl, rw_00BA1F70(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
