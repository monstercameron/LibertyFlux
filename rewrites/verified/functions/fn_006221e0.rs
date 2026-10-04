// original: 0x006221e0 net_entry_copy_head
/// Copy an entry's head fields and its extended block.
///
/// Copies words +0/+4, converts the +0x8 sub-block through the shared
/// helper, copies the scattered words at +0x78/+0x80/+0x88/+0x8c and the
/// half-words at +0x7c/+0x84, then the 0x80-dword block at +0x90.
/// Returns `this`.
export!(thiscall, rw_006221e0(this: u32, src: u32) -> u32 {
    unsafe {
        let dst = this as *mut u8;
        let s = src as *const u8;
        (dst as *mut u32).write_unaligned((s as *const u32).read_unaligned());
        (dst.add(4) as *mut u32)
            .write_unaligned((s.add(4) as *const u32).read_unaligned());
        let _: u32 = callee_thiscall!(
            1,
            u32,
            (dst.add(8) as u32),
            (s.add(8) as u32)
        );
        (dst.add(0x78) as *mut u32)
            .write_unaligned((s.add(0x78) as *const u32).read_unaligned());
        (dst.add(0x7c) as *mut u16)
            .write_unaligned((s.add(0x7c) as *const u16).read_unaligned());
        (dst.add(0x80) as *mut u32)
            .write_unaligned((s.add(0x80) as *const u32).read_unaligned());
        (dst.add(0x84) as *mut u16)
            .write_unaligned((s.add(0x84) as *const u16).read_unaligned());
        (dst.add(0x88) as *mut u32)
            .write_unaligned((s.add(0x88) as *const u32).read_unaligned());
        (dst.add(0x8c) as *mut u32)
            .write_unaligned((s.add(0x8c) as *const u32).read_unaligned());
        core::ptr::copy_nonoverlapping(
            s.add(0x90) as *const u32,
            dst.add(0x90) as *mut u32,
            0x80,
        );
        this
    }
});
