// original: 0x00a02280 SWAP_NEAREST_BUILDING_MODEL
/// Swaps the nearest building model. Forwards four float coordinates and two model arguments to the engine helper.
///
/// Native handler: the script VM passes one pointer to a call context.
/// The context holds the return-slot pointer at offset 0 and the argument
/// array pointer at offset 8. Integer arguments pass through as raw words;
/// float arguments pass through bitwise.
checker_rt::export!(cdecl, rw_00a02280(ctx: *const u8) -> u32 {
    unsafe {
        let argv = *((ctx.add(8)) as *const *const u32);
        let f0 = *((argv.add(0)) as *const f32);
        let f1 = *((argv.add(1)) as *const f32);
        let f2 = *((argv.add(2)) as *const f32);
        let f3 = *((argv.add(3)) as *const f32);
        let a4 = *argv.add(4);
        let a5 = *argv.add(5);
        checker_rt::callee_cdecl!(1, u32, f32::to_bits(f0), f32::to_bits(f1), f32::to_bits(f2), f32::to_bits(f3), a4, a5)
    }
});
