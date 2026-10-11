// original: 0x00DE4260 UIRowLayout::vf141

/// Advance the current child index, wrapping to the first or last child according to the parent's wrap callback. Visit children in order until one reports readiness or the unsigned child count is exhausted. If the chosen index differs from the current index, notify the parent with the chosen index followed by enable word `1` and return one; otherwise return zero. The count and indices are unsigned 32-bit callback results, and readiness is the low byte returned by a child virtual method selected by the parent's mode flag.
lf_checker_rt::export!(thiscall, rw_00de4260(this_ptr: u32) -> u32 {
    unsafe fn parent_method0(parent: u32, slot: u32) -> u32 {
        let table = (parent as *const u32).read_unaligned();
        let target = ((table + slot) as *const u32).read_unaligned();
        let method: unsafe extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        method(parent)
    }
    unsafe fn parent_method1(parent: u32, slot: u32, argument: u32) -> u32 {
        let table = (parent as *const u32).read_unaligned();
        let target = ((table + slot) as *const u32).read_unaligned();
        let method: unsafe extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(target as usize);
        method(parent, argument)
    }
    unsafe fn parent_method2(parent: u32, slot: u32, first: u32, second: u32) -> u32 {
        let table = (parent as *const u32).read_unaligned();
        let target = ((table + slot) as *const u32).read_unaligned();
        let method: unsafe extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(target as usize);
        method(parent, first, second)
    }
    unsafe fn child_ready(child: u32, slot: u32) -> u8 {
        let table = (child as *const u32).read_unaligned();
        let target = ((table + slot) as *const u32).read_unaligned();
        let method: unsafe extern "thiscall" fn(u32) -> u8 = core::mem::transmute(target as usize);
        method(child)
    }
    const COUNT_SLOT: u32 = 0x1D4;
    const INDEX_SLOT: u32 = 0x21C;
    const WRAP_SLOT: u32 = 0x228;
    const CHILD_SLOT: u32 = 0x1E0;
    const PARENT_FLAG: u32 = 0x1F0;
    const LOW_CHILD_READY: u32 = 0x0C;
    const HIGH_CHILD_READY: u32 = 0x10;
    const PARENT_UPDATE_SLOT: u32 = 0x22C;
    let count = unsafe { parent_method0(this_ptr, COUNT_SLOT) };
    let mut candidate = unsafe { parent_method0(this_ptr, INDEX_SLOT) }.wrapping_add(1);
    if count == 0 { return 0; }
    if candidate >= count {
                candidate = if unsafe { parent_method0(this_ptr, WRAP_SLOT) as u8 } != 0 { 0 } else { count.wrapping_sub(1) };
            }
    let mut examined = 0u32;
    loop {
        examined = examined.wrapping_add(1);
        let parent_flag = unsafe { ((this_ptr + PARENT_FLAG) as *const u8).read() };
        let child_ready_slot = if parent_flag != 0 { HIGH_CHILD_READY } else { LOW_CHILD_READY };
        let child = unsafe { parent_method1(this_ptr, CHILD_SLOT, candidate) };
        let ready = unsafe { child_ready(child, child_ready_slot) };
        if ready != 0 {
                    let current_index = unsafe { parent_method0(this_ptr, INDEX_SLOT) };
                    if current_index != candidate {
                        let _ignored = unsafe { parent_method2(this_ptr, PARENT_UPDATE_SLOT, candidate, 1) };
                        return 1;
                    }
                    return 0;
        }
        if examined >= count { return 0; }
        candidate = candidate.wrapping_add(1);
        if candidate >= count {
            candidate = if unsafe { parent_method0(this_ptr, WRAP_SLOT) as u8 } != 0 { 0 } else { count.wrapping_sub(1) };
        }
    }
});
