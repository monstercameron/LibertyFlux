// original: 0x008766B0 rage::crmtRequestBlend::vf2


/// If both state words at +0x14 and +0x18 are zero, return the second
/// incoming stack argument. Otherwise allocate a blend result from that
/// argument, configure it with the byte mode at +0x24, child pointers at
/// +0x28/+0x2c, and f32 weights at +0x1c/+0x20 in XMM1/XMM2, then update
/// this request with both incoming arguments and the allocation. The method
/// is thiscall with two stack words and returns the selected result pointer.
lf_checker_rt::export!(thiscall, rw_008766b0(this: u32, first_argument: u32, allocation_key: u32) -> u32 {
    const STATE_A: u32 = 0x14;
    const STATE_B: u32 = 0x18;
    const MODE: u32 = 0x24;
    const WEIGHT_A: u32 = 0x1c;
    const WEIGHT_B: u32 = 0x20;
    const CHILD_A: u32 = 0x28;
    const CHILD_B: u32 = 0x2c;
    const ALLOCATE: u32 = 1;
    const CONFIGURE: u32 = 2;
    const UPDATE: u32 = 3;

    unsafe {
        let first_state = ((this + STATE_A) as *const u32).read_unaligned();
        let second_state = ((this + STATE_B) as *const u32).read_unaligned();
        if first_state == 0 && second_state == 0 { return allocation_key; }

    let allocated = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, allocation_key);
    let mode = ((this + MODE) as *const u8).read();
    let first_child = ((this + CHILD_A) as *const u32).read_unaligned();
    let second_child = ((this + CHILD_B) as *const u32).read_unaligned();
    let first_weight = ((this + WEIGHT_A) as *const u32).read_unaligned();
    let second_weight = ((this + WEIGHT_B) as *const u32).read_unaligned();

    let _ = lf_checker_rt::callee_thiscall!(
        CONFIGURE, u32, allocated, u32::from(mode), first_child, second_child,
        first_weight, second_weight,
    );
    let _ = lf_checker_rt::callee_thiscall!(UPDATE, u32, this, first_argument,
                                             allocation_key, allocated);
    allocated
    }
});
