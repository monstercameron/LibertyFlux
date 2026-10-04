// original: 0x00625730 pending_dispatch_drain
// Drain loop: while the object reports pending work, load the indirect worker
// from its data-table slot and run it over the payload; when it hands the
// object back, run the frame-scoped follow-up (if installed) and the
// completion callback. Returns the last callback answer, 0 when idle on entry
// (entry residue, pinned to 0 by the contract).
export!(thiscall, rw_00625730(obj: u32) -> u32 {
    unsafe {
        const PENDING_OFF: u32 = 8;
        const FOLLOWUP_OFF: u32 = 0x0c;
        const TABLE_SLOT: u32 = 0x00e731ec;
        const PAYLOAD_ARG_OFF: u32 = 0x10;
        type WorkerFn = extern "stdcall" fn(u32, u32, u32) -> u32;
        if *((obj.wrapping_add(PENDING_OFF)) as *const u32) == 0 {
            return 0;
        }
        let worker: WorkerFn = core::mem::transmute(
            (*(global::<u32>(TABLE_SLOT) as *const u32)) as usize,
        );
        let mut last: u32 = 0;
        loop {
            let payload: u32 = *(obj as *const u32);
            let mut slot: u32 = payload;
            if payload != 0 {
                let back: u32 = worker(payload.wrapping_add(PAYLOAD_ARG_OFF), 0, obj);
                last = back;
                if back != 0 && back == obj {
                    if *((obj.wrapping_add(FOLLOWUP_OFF)) as *const u32) != 0 {
                        let _followed: u32 =
                            callee_stdcall!(2, u32, (&mut slot as *mut u32) as u32);
                    }
                    last = callee_thiscall!(3, u32, obj, payload);
                }
            }
            if *((obj.wrapping_add(PENDING_OFF)) as *const u32) == 0 {
                break;
            }
        }
        last
    }
});
