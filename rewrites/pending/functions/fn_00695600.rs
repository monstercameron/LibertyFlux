// original: 0x00695600 anim_rebase_block
/// Rebase an animation pointer block by asking the slide helper per entry.
///
/// Adjusts the block base and every non-null entry by the helper's answer
/// for that pointer. Returns the block pointer.
export!(thiscall, rs80_695600(this: *mut u8, _unused: u32) -> u32 {
    unsafe {
        let base = *((this).add(0) as *const u32);
        if base != 0 {
            let d: u32 = callee_stdcall!(1, u32, base);
            *((this).add(0) as *mut u32) = base.wrapping_add(d);
        }
        let count = *((this).add(4) as *const u16) as usize;
        let arr = *((this).add(0) as *const u32) as *mut u32;
        for i in 0..count {
            let e = *arr.add(i);
            if e != 0 {
                let d: u32 = callee_stdcall!(1, u32, e);
                *arr.add(i) = e.wrapping_add(d);
            }
        }
        this as u32
    }
});
