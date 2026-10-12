// original: 0x00934480 content_live_slots_activate

/// Refresh the live-slot owner, then process each slot whose present and
/// active flags are set and whose mode byte is not the empty marker. For each
/// eligible record, pass a temporary 28-word work area through the scripted
/// helpers using a 16-word scratch snapshot, publish its id byte to the shared
/// worker global, and run the streaming reset helper. The 16-bit slot count is zero-extended; the return
/// value is that count. The receiver is a thiscall object pointer.
lf_checker_rt::export!(thiscall, rw_00934480(this: *mut u8) -> u32 {
    const ARRAY_POINTER_OFFSET: usize = 0x144;
    const SLOT_COUNT_OFFSET: usize = 0x148;
    const SLOT_STRIDE: u32 = 0x22C;
    const PRESENT_FLAG_OFFSET: u32 = 0x225;
    const ACTIVE_FLAG_OFFSET: u32 = 0x226;
    const EMPTY_MODE: u8 = 0xFF;
    const MODE_OFFSET: u32 = 0x227;
    const ID_OFFSET: u32 = 0x229;
    const SHARED_WORKER_VA: u32 = 0x011D_6FE0;
    const REFRESH_CALLEE: u32 = 1;
    const SET_OWNER_CALLEE: u32 = 2;
    const RESOLVE_WORK_CALLEE: u32 = 3;
    const BUILD_WORK_CALLEE: u32 = 4;
    const SUBMIT_WORK_CALLEE: u32 = 5;
    const RESET_STREAMING_CALLEE: u32 = 6;

    unsafe {
        let receiver = this as u32;
        lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, receiver);
        lf_checker_rt::callee_thiscall!(SET_OWNER_CALLEE, u32, receiver);

        let count = u32::from(
            this.add(SLOT_COUNT_OFFSET).cast::<u16>().read_unaligned(),
        );
        let slots = this.add(ARRAY_POINTER_OFFSET).cast::<u32>().read_unaligned();
        let mut work_area = core::mem::MaybeUninit::<[u32; 16]>::uninit();
        let work_pointer = work_area.as_mut_ptr().cast::<u32>() as u32;

        let mut index = 0u32;
        while index < count {
            let record = slots.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
            let active = (record.wrapping_add(ACTIVE_FLAG_OFFSET) as *const u8).read() != 0;
            let present = (record.wrapping_add(PRESENT_FLAG_OFFSET) as *const u8).read() != 0;
            let mode = (record.wrapping_add(MODE_OFFSET) as *const u8).read();

            if active && present && mode != EMPTY_MODE {
                let record_id = (record.wrapping_add(ID_OFFSET) as *const u8).read();
                lf_checker_rt::callee_thiscall!(RESOLVE_WORK_CALLEE, u32, work_pointer);
                lf_checker_rt::callee_thiscall!(
                    BUILD_WORK_CALLEE,
                    u32,
                    work_pointer,
                    0u32,
                    u32::from(record_id),
                    1u32,
                    0x20u32,
                    0u32,
                    u32::from(mode),
                    0u32,
                    0u32
                );
                lf_checker_rt::callee_cdecl!(SUBMIT_WORK_CALLEE, u32, work_pointer);

                lf_checker_rt::global::<u32>(SHARED_WORKER_VA)
                    .write_unaligned(u32::from(record_id));
                lf_checker_rt::callee_cdecl!(RESET_STREAMING_CALLEE, u32,);
            }
            index = index.wrapping_add(1);
        }

        count
    }
});
