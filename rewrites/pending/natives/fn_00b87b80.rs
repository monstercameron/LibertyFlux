// original: 0x00b87b80 SET_INSTANT_WIDESCREEN_BORDERS
/// Native handler `SET_INSTANT_WIDESCREEN_BORDERS`: toggle instant widescreen borders: forward the flag coerced to 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00b87b80(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        // The original coerces this flag with a byte-wide setnz over the
        // incoming context slot, so the pushed dword keeps the slot's high
        // bytes; v2 port: push the bare flag, mask the arg in the contract.
        // v2 port: push the bare flag; the slot-residue high byte is masked in the contract (call_skip).
        let flag0 = u32::from(*args.add(0) != 0);
        let ans = callee_cdecl!(1, u32, flag0,);
        ans
    }
});
