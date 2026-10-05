// original: 0x00a91b70 stream_record_make

/// Makes a record through the manager and stamps its header.
///
/// Calls the maker (callee 1, thiscall) with the manager at file VA
/// 0x12FB258 and `a1`; its answer is the new record. Initializes it
/// through callee 2 (thiscall) with `a2`, stamps the header (1 as a word
/// at `+0x54`, 0 as a byte at `+0x56`, 0 at `+0x58`, -1 at `+0x50`) and
/// registers it (callee 3, thiscall) with the manager. Returns the
/// register answer. Straight line, three calls, one global.
/// Original: 0x00A91B70 (cdecl, two stack words), 66 bytes (the batch
/// lists 14, ending mid-instruction; the ret and the int3 padding after
/// it fix the true extent).
lf_checker_rt::export!(cdecl, rw_00a91b70(a1: u32, a2: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x12FB258;
        const TAG_W_OFF: u32 = 0x54;
        const TAG_B_OFF: u32 = 0x56;
        const ZERO_OFF: u32 = 0x58;
        const LINK_OFF: u32 = 0x50;
        const MAKE: u32 = 1;
        const INIT: u32 = 2;
        const REGISTER: u32 = 3;
        let mgr = lf_checker_rt::global::<u32>(MGR).read();
        let rec: u32 = lf_checker_rt::callee_thiscall!(MAKE, u32, mgr, a1);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, rec, a2);
        (rec.wrapping_add(TAG_W_OFF) as *mut u16).write_unaligned(1);
        (rec.wrapping_add(TAG_B_OFF) as *mut u8).write(0);
        (rec.wrapping_add(ZERO_OFF) as *mut u32).write_unaligned(0);
        (rec.wrapping_add(LINK_OFF) as *mut u32).write_unaligned(0xFFFF_FFFF);
        let mgr2 = lf_checker_rt::global::<u32>(MGR).read();
        let ans: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, mgr2, rec);
        ans
    }
});
