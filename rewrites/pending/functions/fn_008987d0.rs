// original: 0x008987d0 audio_params_apply_scaled
/// Applies one packed audio parameter record to a voice object.
///
/// Runs the voice setup call with the shared default control value, then
/// scales five packed 16-bit parameters by the global scale factor into the
/// object's float slots, copies four raw words across, and stores the record
/// tail dword, which is also the return value.
export!(thiscall, rw_008987d0(this: u32, src: u32) -> u32 {
    unsafe {
        let default_ctl = core::ptr::read_unaligned(global::<u32>(0x115D9FC));
        callee_thiscall!(1, u32, this, default_ctl, 0);
        let scale: f32 = core::ptr::read_unaligned(global::<f32>(0xFE870C));
        let raw = |off: u32| -> u16 {
            core::ptr::read_unaligned(src.wrapping_add(off) as *const u16)
        };
        // Four scaled parameters land in consecutive float slots; the fifth
        // (offset 0x19) lands in the earlier slot 0x0c.
        let slots: [(u32, u32); 5] = [
            (0x1b, 0x10),
            (0x1d, 0x14),
            (0x1f, 0x18),
            (0x21, 0x1c),
            (0x19, 0x0c),
        ];
        for (src_off, dst_off) in slots {
            let scaled = (raw(src_off) as f32) * scale;
            core::ptr::write_unaligned(this.wrapping_add(dst_off) as *mut f32, scaled);
        }
        let words: [(u32, u32); 4] =
            [(0x0f, 0x20), (0x11, 0x22), (0x13, 0x24), (0x09, 0x6e)];
        for (src_off, dst_off) in words {
            core::ptr::write_unaligned(
                this.wrapping_add(dst_off) as *mut u16,
                raw(src_off),
            );
        }
        let tail: u32 = core::ptr::read_unaligned(src.wrapping_add(5) as *const u32);
        core::ptr::write_unaligned(this.wrapping_add(0x4c) as *mut u32, tail);
        tail
    }
});
