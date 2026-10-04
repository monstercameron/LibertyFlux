// original: 0x00BA0250 IS_PED_RETREATING
// IS_PED_RETREATING: forward the ped handle; store the answer's low byte
// through the return slot. Returns the slot.
export!(cdecl, rw_00BA0250(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let retreating = callee_cdecl!(1, u32, *a) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = retreating;
        slot as u32
    }
});
