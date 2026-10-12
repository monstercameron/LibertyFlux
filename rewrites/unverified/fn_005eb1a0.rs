// original: 0x005EB1A0 CTaskComplexSearchForPedOnFoot::CTaskComplexSearchForPedOnFoot_3

/// Initialize a search-for-ped-on-foot task from three source words and a radius.
///
/// ECX points to the destination, the stack argument points to three source
/// words, and XMM2's low word supplies the float stored at offset `0x30`. The
/// base task constructor is called first. This constructor then stores its
/// relocated vtable pointer, copies the three source words to offsets `0x20`,
/// `0x24` and `0x28`, clears byte `0x34`, and clears dwords `0x40`, `0x44` and
/// `0x48`. It returns the destination in EAX. The base constructor is scripted
/// by the contract, so its internal initialization is outside this proof.
lf_checker_rt::export!(thiscall, rw_005eb1a0(this: *mut u8, source: *const u8) -> *mut u8 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const TASK_VTABLE_VA: u32 = 0x00fe137c;

    unsafe {
        let this_address = this as u32;
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this_address);
        this.cast::<u32>()
            .write_unaligned(lf_checker_rt::relocated(TASK_VTABLE_VA));

        let source_word_0 = source.cast::<u32>().read_unaligned();
        let source_word_1 = source.add(4).cast::<u32>().read_unaligned();
        let source_word_2 = source.add(8).cast::<u32>().read_unaligned();
        this.add(0x20).cast::<u32>().write_unaligned(source_word_0);
        this.add(0x24).cast::<u32>().write_unaligned(source_word_1);
        this.add(0x28).cast::<u32>().write_unaligned(source_word_2);

        let radius_bits = lf_checker_rt::xmm_word(2, 0);
        this.add(0x30).cast::<u32>().write_unaligned(radius_bits);
        this.add(0x34).write(0);
        this.add(0x40).cast::<u32>().write_unaligned(0);
        this.add(0x44).cast::<u32>().write_unaligned(0);
        this.add(0x48).cast::<u32>().write_unaligned(0);
        this
    }
});
