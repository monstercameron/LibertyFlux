// original: 0x008f52c0 input_unit_flags_copy (proposed)

/// Copy the four per-unit state bytes from one device object to another.
///
/// Both `dst` and `src` are input device objects. The copied bytes are the
/// unit block at `+0x3288`: seen-9 (`+0x3289`), seen-B (`+0x328B`), armed
/// (`+0x328C`) and marked (`+0x328D`), in the order 89, 8B, 8D, 8C. Each is
/// moved as an unsigned byte, so the other bytes of the objects are
/// untouched. EAX holds the last byte (zero-extended) at return.
///
/// Thiscall: destination in ECX, source as one stack word, callee cleans 4.
lf_checker_rt::export!(thiscall, rw_008f52c0(dst: u32, src: u32) -> u32 {
    unsafe {
        const FLAG_OFFS: [u32; 4] = [0x3289, 0x328b, 0x328d, 0x328c];
        let mut last = 0u8;
        for o in FLAG_OFFS {
            last = ((src + o) as *const u8).read();
            ((dst + o) as *mut u8).write(last);
        }
        last as u32
    }
});
