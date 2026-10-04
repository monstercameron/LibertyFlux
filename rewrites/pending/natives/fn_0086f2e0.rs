// original: 0x0086f2e0 VMAG2
/// Script native `VMAG2` (hash 0x787206F8).
///
/// Computes the squared length of a three-component vector inline: reads
/// three floats from the argument array and stores
/// x*x + y*y + z*z into the return slot. No engine call is made;
/// the arithmetic order matches the original exactly so results agree
/// bit-for-bit.
export!(cdecl, rw_0086f2e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const f32;
        let slot = *(ctx as *const u32) as *mut f32;
        let x = *args;
        let y = *args.add(1);
        let z = *args.add(2);
        *slot = x * x + y * y + z * z;
        slot as u32
    }
});
