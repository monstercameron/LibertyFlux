// original: 0x00ba01d0 IS_PED_LOOKING_AT_OBJECT
/// Script native `IS_PED_LOOKING_AT_OBJECT` (hash 0x5DD231A2).
///
/// Same as the car variant but against the object pool with object type bits.
///
/// Script arguments: self:Char: ?, object:Object: ?.
///
/// Forwards arg0 (integer/handle), arg1 (integer/handle) to the engine routine.
/// The engine's byte answer is zero-extended into the return slot.
export!(cdecl, rw_00ba01d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const *mut u32);
        let arg0 = *args.add(0);
        let arg1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, arg0, arg1, );
        *slot = answer & 0xFF;
        slot as u32
    }
});
