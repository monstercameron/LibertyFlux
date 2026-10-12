// original: 0x00877710 rage::crmtComposer::vf6

/// Clear a composer link flag and its caller-provided status word.
///
/// The linked-state byte at `this + 0x11` and the first word behind the
/// output pointer are both set to zero. The function returns the byte value
/// one in AL; upper EAX bits are incidental pointer residue.
///
/// Original calling convention: thiscall with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00877710(this: u32, output: u32) -> u32 {
    const LINKED_FLAG: u32 = 0x11;
    unsafe {
        (this.wrapping_add(LINKED_FLAG) as *mut u8).write(0);
        (output as *mut u32).write_unaligned(0);
    }
    1
});
