// original: 0x00a01920 RENDER_WEAPON_PICKUPS_BIGGER
/// Native handler `RENDER_WEAPON_PICKUPS_BIGGER`: scale up weapon pickups: forward the enable flag coerced to 0/1.
///
/// The single argument is the native call context: offset 0 holds the
/// return-slot pointer and offset 8 the script argument array.
export!(cdecl, rw_00a01920(ctx: u32) -> u32 {
    unsafe {
        let args = *((ctx + 8) as *const u32) as *const u32;
        // The original coerces this flag with a byte-wide setnz over the
        // incoming context slot, so the pushed dword keeps the slot's high
        // bytes; reproduce that shape exactly.
        let flag0 = (ctx & 0xFFFF_FF00) | u32::from(*args.add(0) != 0);
        let ans = callee_cdecl!(1, u32, flag0,);
        ans
    }
});
