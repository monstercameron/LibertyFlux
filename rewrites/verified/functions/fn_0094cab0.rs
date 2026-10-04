// original: 0x0094cab0 node_array_init16
/// Initialise 16 nodes with stride 32: flag byte 0, kind word 1, link
/// 0xFFFFFFFF and 24 zero bytes. Byte 1 of each node is intentionally left
/// untouched. Returns the end pointer (start + 0x204), which the original
/// leaves in EAX.
export!(thiscall, rw_0094cab0(obj: *mut u8) -> u32 {
    unsafe {
        for i in 0..16usize {
            let e = obj.add(i * 0x20);
            *e = 0;
            *(e.add(2) as *mut u16) = 1;
            *(e.add(4) as *mut u32) = 0xFFFF_FFFF;
            core::ptr::write_bytes(e.add(8), 0, 24);
        }
        (obj as u32).wrapping_add(0x204)
    }
});
