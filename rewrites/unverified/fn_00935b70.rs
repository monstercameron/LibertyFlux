// original: 0x00935B70 synchronize_network_state_globals

/// Synchronize two global words from the source global when the receiver's
/// byte at `+0x169` is set. Otherwise, set the receiver's word at `+0x167`,
/// call the associated-object helper and the receiver reset helper, then
/// clear both destination globals. The reset path calls the associated-object
/// helper first with `this + 0x158` and calls the receiver helper second. This
/// is a void-style thiscall routine with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00935b70(this: u32) -> () {
    unsafe {
        const SYNC_FLAG: u32 = 0x169;
        const RECEIVER_FLAG: u32 = 0x167;
        const ASSOCIATED_OBJECT: u32 = 0x158;
        const SOURCE_GLOBAL: u32 = 0x011d_6fd4;
        const DESTINATION_GLOBAL_A: u32 = 0x011d_6fd8;
        const DESTINATION_GLOBAL_B: u32 = 0x0116_0c08;

        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }

        if read_u8(this.wrapping_add(SYNC_FLAG)) != 0 {
            let source = unsafe { lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned() };
            unsafe { lf_checker_rt::global::<u32>(DESTINATION_GLOBAL_A).write_unaligned(source) };
            unsafe { lf_checker_rt::global::<u32>(DESTINATION_GLOBAL_B).write_unaligned(source) };
        } else {
            write_u16(this.wrapping_add(RECEIVER_FLAG), 1);
            let _ = lf_checker_rt::callee_thiscall!(
                1,
                u32,
                this.wrapping_add(ASSOCIATED_OBJECT)
            );
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
            unsafe { lf_checker_rt::global::<u32>(DESTINATION_GLOBAL_A).write_unaligned(0) };
            unsafe { lf_checker_rt::global::<u32>(DESTINATION_GLOBAL_B).write_unaligned(0) };
        }
    }
});
