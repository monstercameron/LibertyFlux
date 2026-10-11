// original: 0x009BB180 CCamScriptInstruction_SetViewportMirrored::vf2

/// Set the mirrored bit on the viewport matching this instruction's key.
///
/// The instruction key is at `this + 0x08`; its mirror flag is the low bit of
/// the byte at `this + 0x0C`. The global viewport list is a pointer to a
/// sequence of object pointers with a 16-bit count. The first object whose
/// identifier at `+0x53C` matches the key has bit 2 of its flags byte at
/// `+0x558` replaced by the instruction flag. Other flag bits are preserved.
/// An empty list or missing key changes nothing. The original is a void
/// thiscall method and does not define a value in EAX.
lf_checker_rt::export!(thiscall, rw_009bb180(this: u32) -> () {
    unsafe {
        const VIEWPORTS_VA: u32 = 0x0118_d7f0;
        const VIEWPORT_COUNT_VA: u32 = 0x0118_d7f4;
        const VIEWPORT_ID: u32 = 0x53c;
        const VIEWPORT_FLAGS: u32 = 0x558;
        const MIRRORED_BIT: u8 = 0x04;

        let wanted = (this.wrapping_add(8) as *const u32).read_unaligned();
        let enabled = ((this.wrapping_add(0x0c) as *const u8).read() & 1) != 0;
        let list = lf_checker_rt::global::<u32>(VIEWPORTS_VA).read_unaligned();
        let count = (lf_checker_rt::global::<u16>(VIEWPORT_COUNT_VA)).read_unaligned();
        for index in 0..u32::from(count) {
            let viewport = ((list.wrapping_add(index * 4)) as *const u32).read_unaligned();
            let identifier = (viewport.wrapping_add(VIEWPORT_ID) as *const u32).read_unaligned();
            if identifier == wanted {
                let flags = (viewport.wrapping_add(VIEWPORT_FLAGS) as *mut u8).read();
                let next = if enabled { flags | MIRRORED_BIT } else { flags & !MIRRORED_BIT };
                (viewport.wrapping_add(VIEWPORT_FLAGS) as *mut u8).write(next);
                break;
            }
        }
    }
});
