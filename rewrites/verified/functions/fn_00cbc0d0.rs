// original: 0x00CBC0D0 CTaskSimpleMoveGoToPointOnRoute::vf10

/// Replace the float at offset 0x8c only when it is unequal to the one stack argument or either value is unordered; equal values, including signed zero pairs, leave memory untouched. The thiscall method pops one stack word and has no defined return value.
lf_checker_rt::export!(thiscall, rw_00cbc0d0(this: u32, incoming: f32) -> () {
unsafe {
        const VALUE: u32 = 0x8c;
        let stored = ((this + VALUE) as *const f32).read_unaligned();
        if stored != incoming {
            ((this + VALUE) as *mut f32).write_unaligned(incoming);
        }
    }
});
