// original: 0x009BB110 CCamScriptInstruction_SetViewportDest::vf2

/// Set the destination rectangle for a named viewport.
///
/// After looking up the viewport named at `this + 0x08`, the handler passes
/// two frame-local pairs to the destination helper: the origin pair is the
/// f32 words at `+0x0C` and `+0x10`; the far-corner pair adds those values,
/// in that operand order, to the words at `+0x14` and `+0x18`. It also passes
/// the words at `+0x1C` and `+0x20`. The two pointer values
/// are stack-local addresses, so the contract skips those addresses and
/// compares both pointed-to pairs at the call. The helper's return is the
/// handler's EAX result.
lf_checker_rt::export!(thiscall, rw_009bb110(this: u32) -> u32 {
    unsafe {
        const MANAGER: u32 = 0x0128_e400;
        const LOOKUP: u32 = 1;
        const SET_DESTINATION: u32 = 2;

        let key = (this.wrapping_add(8) as *const u32).read_unaligned();
        let viewport = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            lf_checker_rt::relocated(MANAGER),
            key,
        );
        let x = f32::from_bits((this.wrapping_add(0x0c) as *const u32).read_unaligned());
        let y = f32::from_bits((this.wrapping_add(0x10) as *const u32).read_unaligned());
        let right = f32::from_bits((this.wrapping_add(0x14) as *const u32).read_unaligned());
        let bottom = f32::from_bits((this.wrapping_add(0x18) as *const u32).read_unaligned());
        let far_x = core::hint::black_box(right) + core::hint::black_box(x);
        let far_y = core::hint::black_box(bottom) + core::hint::black_box(y);
        let origin = [x.to_bits(), y.to_bits()];
        let far_corner = [far_x.to_bits(), far_y.to_bits()];
        let extra = (this.wrapping_add(0x1c) as *const u32).read_unaligned();

        lf_checker_rt::callee_thiscall!(
            SET_DESTINATION,
            u32,
            viewport,
            origin.as_ptr() as u32,
            far_corner.as_ptr() as u32,
            extra,
            (this.wrapping_add(0x20) as *const u32).read_unaligned(),
        )
    }
});
