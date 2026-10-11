// original: 0x00CBC000 CTaskSimpleMoveGoToPointOnRoute::vf9

/// When the low byte of the third stack argument is nonzero, always copy the second argument's four-word position into this at offset 0x2c and replace the 0x0e00 flag bits at offset 0xb0 with 0x0100. Otherwise perform both updates only when any of the first three coordinates is unequal or unordered. The first stack argument is unused; the thiscall method pops three words.
lf_checker_rt::export!(thiscall, rw_00cbc000(this: u32, _unused: u32, source: u32, force: u32) -> () {
unsafe {
        const POSITION: u32 = 0x2c;
        const FLAGS: u32 = 0xb0;
        let current = (this + POSITION) as *const f32;
        let x = current.read_unaligned();
        let y = ((this + POSITION + 4) as *const f32).read_unaligned();
        let z = ((this + POSITION + 8) as *const f32).read_unaligned();
        let new_x = (source as *const f32).read_unaligned();
        let new_y = ((source + 4) as *const f32).read_unaligned();
        let new_z = ((source + 8) as *const f32).read_unaligned();
        if force as u8 != 0 || x != new_x || y != new_y || z != new_z {
            for offset in [0u32, 4, 8, 12] {
                let word = ((source + offset) as *const u32).read_unaligned();
                ((this + POSITION + offset) as *mut u32).write_unaligned(word);
            }
            let flags = (this + FLAGS) as *mut u32;
            let value = flags.read_unaligned();
            flags.write_unaligned((value & 0xffff_87ff) | 0x100);
        }
    }
});
