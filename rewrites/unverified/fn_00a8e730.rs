// original: 0x00A8E730 publish_vector_run_kernel (proposed)

/// Publish a vector to the shared slots and run the kernel.
///
/// The untagged sibling of the tagged kernel runner: the four floats at
/// `*vec` go to the shared globals (no tag, no busy byte) and the kernel
/// runs with the same five words (out-slot pointer, constant entry,
/// vector-slot pointer, `0x100`, 5), answering through the out-slot. The
/// second stack word is stored to a dead frame slot and never observed.
///
/// Original: cdecl, two stack words, returns u32 in EAX. One outgoing
/// call (cdecl, five words; the two frame pointers are skipped and their
/// contents snapshotted).
lf_checker_rt::export!(cdecl, rw_00A8E730(vec: u32, _k: u32) -> u32 {
    unsafe {
        const GX: u32 = 0x12fb230;
        const GY: u32 = 0x12fb234;
        const GZ: u32 = 0x12fb238;
        const GW: u32 = 0x12fb23c;
        // Relocated like the original's adjusted immediate (see sibling).
        const ENTRY_FILE_VA: u32 = 0xa8e7c0;
        const KERNEL: u32 = 1;
        let entry = lf_checker_rt::relocated(ENTRY_FILE_VA);
        let x = ((vec + 0) as *const u32).read_unaligned();
        let y = ((vec + 4) as *const u32).read_unaligned();
        let z = ((vec + 8) as *const u32).read_unaligned();
        let w = ((vec + 12) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u32>(GX) as *mut u32).write_unaligned(x);
        (lf_checker_rt::global::<u32>(GY) as *mut u32).write_unaligned(y);
        (lf_checker_rt::global::<u32>(GZ) as *mut u32).write_unaligned(z);
        (lf_checker_rt::global::<u32>(GW) as *mut u32).write_unaligned(w);
        let mut answer: u32 = 0;
        let mut block = [x, y, z];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            KERNEL,
            u32,
            &mut block as *mut u32 as u32,
            entry,
            &mut answer as *mut u32 as u32,
            0x100,
            5
        );
        answer
    }
});
