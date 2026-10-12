// original: 0x00646070 ptxEmitRuleStd_wire_field_pointers (proposed)

/// Wire the ten embedded field blocks of a particle-emitter rule to the
/// pointer table at the end of the object: slot `i` receives the address of
/// its field (`this + SRC[i]`). The last source address (`this + 0x4c`) is
/// left in eax. Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00646070(this: u32) -> u32 {
    unsafe {
        const LAST_SRC: u32 = 0x4c;
        // (field offset, pointer-slot offset), in the original's order.
        const SLOTS: [(u32, u32); 10] = [
            (0x24, 0x1b8),
            (0x74, 0x1bc),
            (0x9c, 0x1c0),
            (0xc4, 0x1c4),
            (0xec, 0x1c8),
            (0x114, 0x1cc),
            (0x13c, 0x1d0),
            (0x164, 0x1d4),
            (0x18c, 0x1d8),
            (0x4c, 0x1dc),
        ];
        for pair in SLOTS {
            ((this.wrapping_add(pair.1)) as *mut u32).write_unaligned(this.wrapping_add(pair.0));
        }
        this.wrapping_add(LAST_SRC)
    }
});
