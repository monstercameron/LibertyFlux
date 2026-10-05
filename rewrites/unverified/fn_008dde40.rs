// original: 0x008dde40 CSetupDefLight::vf1 (symbols)

/// Publish this setup light's descriptor block: pick the two global
/// parameters by probing the selector twice, then call the publisher with
/// the ten-word block (tag, five member pointers, three copied parameter
/// words, frame scratch).
///
/// Each probe's low byte chooses between the two globals of its pair; both
/// probes run with the object pointer (the stubs preserve it). The chosen
/// parameters are converted to float and parked in frame scratch past the
/// argument block, so their values never reach the comparison; what the
/// publisher observes is the tag word at `+0x50`, the members at `+0x10`,
/// `+0x20`, `+0x30` and `+0x40`, the parameter words at `+0x54`, `+0x58` and
/// `+0x5c`, and the tag word at `+0x60`. The tenth argument points at
/// uninitialized frame scratch and is skipped without a snapshot.
///
/// Original: thiscall; returns the publisher's answer.
lf_checker_rt::export!(thiscall, rw_008dde40(this: *mut u32) -> u32 {
    unsafe {
        let a1 = lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        let sel0 = if a1 as u8 != 0 {
            *lf_checker_rt::global::<u32>(0x105C87C)
        } else {
            *lf_checker_rt::global::<u32>(0x105C880)
        };
        let a2 = lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        let sel1 = if a2 as u8 != 0 {
            *lf_checker_rt::global::<u32>(0x105C888)
        } else {
            *lf_checker_rt::global::<u32>(0x105C884)
        };
        core::hint::black_box(((sel0 as i32) as f32, (sel1 as i32) as f32));
        let scratch = [0u32; 1];
        lf_checker_rt::callee_cdecl!(
            2,
            u32,
            *this.add(20),
            this.byte_add(0x10) as u32,
            *this.add(21),
            this.byte_add(0x20) as u32,
            this.byte_add(0x30) as u32,
            *this.add(22),
            *this.add(23),
            this.byte_add(0x40) as u32,
            *this.add(24),
            scratch.as_ptr() as u32
        )
    }
});
