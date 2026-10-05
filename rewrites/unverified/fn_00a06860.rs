// original: 0x00a06860 NativeImpl_SET_OBJECT_COORDINATES_2 (native)
/// Move an object to 2D coordinates with a settled depth.
///
/// Resolves `handle` through the object pool. When the read-only limit is
/// ordered-above-or-equal to `hint` the depth filter runs on (x, y, 4) for
/// the depth; otherwise (including NaN hints, matching comiss+jb) the hint
/// is used as-is. The object's positioning slot (+0x7c) then takes the
/// record [x, -10.0, y, depth] with a stale stack word the original pushes
/// from ecx (skipped in the contract: it is stub scratch, not behaviour),
/// and the move is committed. Returns the commit answer. Cdecl.
lf_checker_rt::export!(cdecl, rw_00a06860(handle: u32, x: u32, y: u32, hint: u32) -> u32 {
    unsafe {
        const OBJ_POOL: u32 = 0x01632c60;
        const LIMIT_CONST: u32 = 0x00fe8df8;
        const SLOT: u32 = 0x7c;
        const LOOKUP: u32 = 0;
        const DEPTH: u32 = 1;
        const COMMIT: u32 = 2;
        const POSITION: u32 = 3;
        const NEG10: u32 = 0xc120_0000;
        let pool = (lf_checker_rt::global::<u32>(OBJ_POOL) as *const u32).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(LOOKUP, u32, pool, handle);
        let lim = f32::from_bits(
            (lf_checker_rt::relocated(LIMIT_CONST) as *const u32).read_unaligned());
        let h = f32::from_bits(hint);
        let depth_bits = if lim >= h {
            lf_checker_rt::callee_cdecl!(DEPTH, f32, x, y, 4u32).to_bits()
        } else {
            hint
        };
        let rec = [x, NEG10, y, depth_bits];
        let vtab = (obj as *const u32).read_unaligned();
        let slot = ((vtab + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _: u32 = f(obj, rec.as_ptr() as u32, 0u32, 0u32);
        lf_checker_rt::callee_cdecl!(COMMIT, u32, obj)
    }
});
