/// Proof scope: Only two registry slots are populated; other table populations are untested.
/// Virtual return values other than four are untested; helper bodies are scripted.
/// Full-data snapshot is disabled.
// original: 0x009DD860 model_slot_sweep
/// Visits the model-slot registry and dispatches the per-entry helpers.
///
/// The scan covers every registry slot. For each occupied slot, it conditionally
/// runs the validation helper from the flags and validation result, then calls the
/// virtual slot handler. The final helper runs only when that handler returns 4.
const MODEL_SLOT_TABLE_VA: u32 = 0x1295_CD8;
const MODEL_SLOT_COUNT: u32 = 0x7918;

lf_checker_rt::export!(cdecl, rw_009dd860() -> () {
    unsafe {
        let table = lf_checker_rt::global::<u32>(MODEL_SLOT_TABLE_VA);
        let mut index = 0u32;
        while index < MODEL_SLOT_COUNT {
            let object = table.add(index as usize).read();
            if object != 0 {
                let object_bytes = object as *mut u8;
                let flags = object_bytes.add(0x54).cast::<u16>().read_unaligned();
                if flags != u16::MAX {
                    let global_argument = lf_checker_rt::global::<u32>(0x12B4_13C).read();
                    let answer = lf_checker_rt::callee_cdecl!(
                        1,
                        u32,
                        lf_checker_rt::relocated(0x00E9_7484),
                        index,
                        global_argument,
                    );
                    if answer == 0xFFFF {
                        lf_checker_rt::callee_thiscall!(
                            2,
                            u32,
                            object,
                            lf_checker_rt::relocated(0x00E9_748C),
                        );
                    }
                }

                let vtable = object_bytes.cast::<u32>().read();
                let target = (vtable as *const u8).add(0x0C).cast::<u32>().read();
                let virtual_call: extern "thiscall" fn(u32) -> u8 =
                    core::mem::transmute(target as usize);
                if virtual_call(object) == 4 {
                    let value = object_bytes.add(0x48).cast::<i16>().read_unaligned();
                    let second_global = lf_checker_rt::global::<u32>(0x1032_F58).read();
                    lf_checker_rt::callee_cdecl!(
                        4,
                        u32,
                        value as i32 as u32,
                        second_global,
                    );
                }
            }
            index += 1;
        }
    }
});
