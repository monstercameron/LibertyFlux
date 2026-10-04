// original: 0x009e23e0 portal_vis_pair_publish
/// When the pending portal-visibility pair differs from the live
/// pair, retire the old pair and install the new one through the helper, then
/// publish the new pair. Returns the live first word. (cdecl/0)
export!(cdecl, rw_009e23e0() -> u32 {
    unsafe {
        let mut eax = *global::<u32>(0x12B41A8);
        let mut ecx = *global::<u32>(0x12B41A4);
        // Initialised on every path below; the 0 only satisfies the compiler.
        let mut edx = 0u32;
        let mut changed = eax != ecx;
        if !changed {
            edx = *global::<u32>(0x103B110);
            changed = *global::<u32>(0x103B114) != edx;
        }
        if changed {
            let old_b = *global::<u32>(0x103B114);
            callee_cdecl!(1, u32, eax, old_b, 0);
            edx = *global::<u32>(0x103B110);
            callee_cdecl!(1, u32, ecx, edx, 1);
            ecx = *global::<u32>(0x12B41A4);
            eax = *global::<u32>(0x12B41A8);
            edx = *global::<u32>(0x103B110);
        }
        if ecx != eax {
            eax = ecx;
        }
        *global::<u32>(0x12B41A8) = eax;
        *global::<u32>(0x103B114) = edx;
        eax
    }
});
