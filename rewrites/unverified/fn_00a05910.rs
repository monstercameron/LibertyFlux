// original: 0x00a05910 NativeImpl_IS_OBJECT_ON_SCREEN (native)
/// Test whether an object is on screen, combining two probe floats.
///
/// Resolves `handle` through the object pool and reads the view-family
/// table (dead loads the original keeps). It primes the first probe slot
/// call, takes a heap float from the second slot call (whose x87 result is
/// popped into a dead stack slot), and combines the third and fourth slot
/// results: the flag base is the high byte of the third result's word at
/// +8, and the combiner (taking the out-word plus 0x10 in ecx with the
/// heap float and the fourth result's word at +4 on the stack) forces the
/// flag to 1 when it answers nonzero. Full eax (via movzx). Cdecl.
lf_checker_rt::export!(cdecl, rw_00a05910(handle: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const FAMILY: u32 = 0x0118d818;
        const SLOT_PROBE: u32 = 0x54;
        const SLOT_FLOAT: u32 = 0x58;
        const LOOKUP: u32 = 0;
        const PROBE: u32 = 1;
        const FLOATSRC: u32 = 2;
        const COMBINE: u32 = 3;
        let pool = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, handle);
        let fam = lf_checker_rt::global::<u32>(FAMILY) as *const u32;
        let count = fam.read_unaligned();
        let _tval = fam.add(count as usize).read_unaligned();
        let vtab = (obj as *const u32).read_unaligned();
        let probe_addr = ((vtab + SLOT_PROBE) as *const u32).read_unaligned();
        let float_addr = ((vtab + SLOT_FLOAT) as *const u32).read_unaligned();
        let probe: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(probe_addr as usize);
        let floatsrc: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(float_addr as usize);
        let mut s0 = [0u32; 1];
        let _ = probe(obj, s0.as_mut_ptr() as u32);
        let fptr = floatsrc(obj, 0u32);
        let mut s1 = [0u32; 1];
        let r3 = probe(obj, s1.as_mut_ptr() as u32);
        let f3 = ((r3 + 8) as *const u32).read_unaligned();
        let mut s2 = [0u32; 1];
        let r4 = probe(obj, s2.as_mut_ptr() as u32);
        let f4 = ((r4 + 4) as *const u32).read_unaligned();
        let fa = (fptr as *const u32).read_unaligned();
        let comb: u32 = lf_checker_rt::callee_thiscall!(
            COMBINE, u32, s0[0].wrapping_add(0x10), fa, f4);
        if comb != 0 {
            1
        } else {
            (f3 >> 24) & 0xff
        }
    }
});
