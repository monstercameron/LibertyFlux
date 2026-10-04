// original: 0x00952aa0 vtable_metric_x20
/// Call the object's slot-40 hook; combine two grandchild words times 20.
///
/// Returns 0 when the hook or any link in the chain is null, otherwise
/// `20 * mid[4]`, plus `20 * tail[4]` when the optional tail is present.
export!(cdecl, rw_00952aa0(obj: u32) -> u32 {
    unsafe {
        let vtable = *(obj as *const u32);
        let slot = *((vtable.wrapping_add(0xA0)) as *const u32);
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let root = hook(obj);
        if root == 0 {
            return 0;
        }
        let inner = *((root.wrapping_add(0x64)) as *const u32);
        if inner == 0 {
            return 0;
        }
        let mid = *((inner.wrapping_add(0x160)) as *const u32);
        if mid == 0 {
            return 0;
        }
        let head = *((mid.wrapping_add(0x10)) as *const u32);
        let tail = *((inner.wrapping_add(0x168)) as *const u32);
        let mut out = head.wrapping_mul(20);
        if tail != 0 {
            let extra = *((tail.wrapping_add(0x10)) as *const u32);
            out = out.wrapping_add(extra.wrapping_mul(20));
        }
        out
    }
});
