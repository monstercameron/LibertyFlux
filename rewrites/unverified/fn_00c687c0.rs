// original: 0x00C687C0 request_list_rebuild

/// Resets a local request list, applies the two global slot-request gates,
/// then scans the object's leading slots. Objects carrying the high-slot flag
/// are filtered by the request helper and inserted into the local list when
/// rejected. The list is finalized and its selected value is returned.
lf_checker_rt::export!(thiscall, rw_00c687c0(this: u32) -> u32 {
    unsafe {
        const FIRST_GLOBAL: u32 = 0x012B4138;
        const SECOND_GLOBAL: u32 = 0x012FA644;
        const OBJECT_TABLE: u32 = 0x01295CD8;
        const OBJECT_FLAGS: u32 = 0x124;
        const HIGH_SLOT_FLAG: u32 = 0x20;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_unaligned()
        }

        let mut requests = [0u32; 64];
        let request_buffer = requests.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, request_buffer);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, request_buffer);

        let first = lf_checker_rt::global::<u32>(FIRST_GLOBAL).read();
        let second = lf_checker_rt::global::<u32>(SECOND_GLOBAL).read();
        let loaded = lf_checker_rt::callee_cdecl!(2, u32, second, first) as u8;
        if loaded != 0 {
            let requested = lf_checker_rt::callee_cdecl!(3, u32, second, first) as u8;
            if requested == 0 {
                let _ = lf_checker_rt::callee_thiscall!(4, u32, request_buffer, second);
            }
        }

        let slots = this.wrapping_add(0x404);
        let count = lf_checker_rt::callee_thiscall!(5, u32, slots) as i32;
        if count > 0 {
            let mut index = 0i32;
            while index < count {
                let object_id = lf_checker_rt::callee_thiscall!(6, u32, slots, index as u32);
                let object = lf_checker_rt::global::<u32>(OBJECT_TABLE)
                    .add(object_id as usize)
                    .read();
                if read_u32(object.wrapping_add(OBJECT_FLAGS)) & HIGH_SLOT_FLAG != 0 {
                    let requested = lf_checker_rt::callee_cdecl!(3, u32, object_id, first) as u8;
                    if requested == 0 {
                        let _ = lf_checker_rt::callee_thiscall!(4, u32, request_buffer, object_id);
                    }
                }
                index += 1;
            }
        }

        let result = lf_checker_rt::callee_thiscall!(7, u32, request_buffer);

        // The original passes its frame-derived security cookie in ECX.
        let _ = lf_checker_rt::callee_thiscall!(8, u32, 0);
        result
    }
});
