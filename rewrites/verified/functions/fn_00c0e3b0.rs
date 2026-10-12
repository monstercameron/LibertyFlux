// original: 0x00C0E3B0 UIMusicContainer::vf107
/// Run the selection notification method through virtual slot 9. If the
/// nested music state exists, ask the HUD colour helper for colour 0x3e and
/// store the returned colour word at the nested colour field. Finish by
/// returning the result of virtual slot 6. `this` is in ECX and there are no
/// stack arguments; the called methods and helper are stand-ins in the proof.
lf_checker_rt::export!(thiscall, rw_c0e3b0(this: u32) -> u32 {
    unsafe {
        const NESTED_STATE: u32 = 0x200;
        const NESTED_OWNER: u32 = 0x1fc;
        const NESTED_COLOUR: u32 = 0x1e0;
        const HUD_COLOUR: u32 = 0x3e;

        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 1);
        let nested_state = ((this + NESTED_STATE) as *const u32).read_unaligned();
        if nested_state != 0 {
            let mut output_word = this;
            let output_ptr = (&mut output_word as *mut u32) as u32;
            let colour_ptr = lf_checker_rt::callee_cdecl!(
                2, u32, output_ptr, HUD_COLOUR
            );
            let colour = (colour_ptr as *const u32).read_unaligned();
            let nested_owner = ((nested_state + NESTED_OWNER) as *const u32).read_unaligned();
            ((nested_owner + NESTED_COLOUR) as *mut u32).write_unaligned(colour);
        }
        lf_checker_rt::callee_thiscall!(3, u32, this, 1)
    }
});
