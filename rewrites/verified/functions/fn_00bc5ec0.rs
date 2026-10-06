// original: 0x00bc5ec0 GET_HEIGHT_OF_VEHICLE
/// Read the height of a vehicle above the ground.
///
/// Builds a five-word argument block (three position words from script
/// arguments 1-3, a flag for argument 4, and the context word with its low
/// byte replaced by a flag for argument 5), forwards it in ECX with the
/// vehicle handle (argument 0) to the engine implementation, and stores the
/// returned height in the return slot.
///
/// The contract skips the ECX address and snapshots all five pointed-to
/// words; the stack check is off because the original overwrites the low
/// byte of its own incoming stack slot (the pushed copy is still compared).
export!(cdecl, rw_00bc5ec0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let flag0 = u32::from(*args.add(4) != 0);
        let flag1 = u32::from(*args.add(5) != 0);
        let block = [*args.add(1), *args.add(2), *args.add(3), flag0,
                     (ctx & 0xffffff00) | flag1];
        let height: f32 = callee_thiscall!(1, f32, block.as_ptr() as u32, *args);
        let slot = *(ctx as *const u32);
        *(slot as *mut f32) = height;
        slot
    }
});
