// original: 0x009BA5D0 CCamScriptInstruction_SetLookTargetCam::vf2
/// Point a camera's look target at another camera, by camera kind.
///
/// Resolves the target camera (`this+0x08`) and the source camera
/// (`this+0x0C`), then reads the target's kind through its virtual slot at
/// `+0x28`. Kind 14 forwards the source to `callee 3`, kind 25 to `callee 4`;
/// any other kind (or a null target) does nothing. The second lookup is only
/// reached when the first is non-null.
lf_checker_rt::export!(thiscall, rw_009BA5D0(this: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x128E400;
        const TARGET_ID: u32 = 0x08;
        const SRC_ID: u32 = 0x0C;
        const KIND_SLOT: u32 = 0x28;
        const KIND_A: u32 = 14;
        const KIND_B: u32 = 25;
        const LOOKUP: u32 = 1;
        const KIND_OF: u32 = 2;
        const SET_A: u32 = 3;
        const SET_B: u32 = 4;
        let _ = KIND_OF;
        let mgr = lf_checker_rt::relocated(CAM_MGR);
        let tid = (this.wrapping_add(TARGET_ID) as *const u32).read_unaligned();
        let tgt = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, tid);
        if tgt == 0 {
            return 0;
        }
        let sid = (this.wrapping_add(SRC_ID) as *const u32).read_unaligned();
        let src = lf_checker_rt::callee_thiscall!(LOOKUP, u32, mgr, sid);
        let vtable = (tgt as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let kind = kind_of(tgt);
        if kind == KIND_A {
            lf_checker_rt::callee_thiscall!(SET_A, u32, tgt, src);
        } else if kind == KIND_B {
            lf_checker_rt::callee_thiscall!(SET_B, u32, tgt, src);
        }
        0
    }
});
