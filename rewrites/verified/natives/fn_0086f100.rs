// original: 0x0086f100 PRINTFLOAT2
/// Script native `PRINTFLOAT2` (hash 0x108A527F).
///
/// Forwards three script arguments (two words and one float bit-pattern)
/// to the engine through a function-pointer slot in the data section.
/// No return slot is written.
export!(cdecl, rw_0086f100(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let target = *global::<u32>(0x110B714);
        let engine: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        engine(*args, *args.add(1), *args.add(2))
    }
});
