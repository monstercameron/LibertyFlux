// original: 0x00BC6AF0 IS_CAR_STUCK_ON_ROOF
// IS_CAR_STUCK_ON_ROOF: forward the vehicle handle; store the answer's
// low byte through the return slot. Returns the slot.
export!(cdecl, rw_00BC6AF0(ctx: u32) -> u32 {
    unsafe {
        let a = args_of(ctx);
        let stuck = callee_cdecl!(1, u32, *a) & 0xFF;
        let slot = ret_slot(ctx);
        *slot = stuck;
        slot as u32
    }
});
