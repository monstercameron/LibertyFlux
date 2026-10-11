// original: 0x00D3B2C0 ped_task_replace_animation_clip

/// When the animation clip at `this`+0x18 differs from the replacement, call its virtual reference method and decrement the returned counter byte at +0x15. Notify the linked object when its flags masked by 0x3c0 equal 0x100, then store the replacement. A nonnull replacement receives the same virtual method call and its returned counter byte is incremented. The callee removes one stack argument; the function has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00d3b2c0(this: u32, replacement: u32) -> u32 {
    unsafe {
        const CLIP_SLOT: u32 = 0x18;
        const LINKED_OBJECT: u32 = 0x1c;
        const CLIP_REFERENCE_METHOD: u32 = 1;
        const NOTIFY_LINKED_OBJECT: u32 = 2;
        let slot = this.wrapping_add(CLIP_SLOT);
        let old = (slot as *const u32).read_unaligned();
        if old != replacement {
            if old != 0 {
                let count = lf_checker_rt::callee_thiscall!(CLIP_REFERENCE_METHOD, u32, old);
                let count_byte = (count.wrapping_add(0x15) as *mut u8);
                count_byte.write_unaligned(count_byte.read_unaligned().wrapping_sub(1));
                let linked = (this.wrapping_add(LINKED_OBJECT) as *const u32).read_unaligned();
                if linked != 0 {
                    let flags = (linked.wrapping_add(0x28) as *const u32).read_unaligned();
                    if (flags & 0x3c0) == 0x100 {
                        let _ = lf_checker_rt::callee_thiscall!(NOTIFY_LINKED_OBJECT, u32, linked, old, 0);
                    }
                }
            }
            (slot as *mut u32).write_unaligned(replacement);
            if replacement != 0 {
                let count = lf_checker_rt::callee_thiscall!(CLIP_REFERENCE_METHOD, u32, replacement);
                let count_byte = (count.wrapping_add(0x15) as *mut u8);
                count_byte.write_unaligned(count_byte.read_unaligned().wrapping_add(1));
            }
        }
        0
    }
});
