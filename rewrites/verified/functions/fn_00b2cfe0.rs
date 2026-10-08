/// Proof scope: AL-only return comparison; upper return-register bytes are excluded.
/// The virtual method is a scripted recorder with synthetic object and vtable data.
/// Ten deterministic input classes cover the mapped conditional outcomes analytically.
/// Call-log shapes do not constitute instrumentation of internal branches.
/// Real helper effects and caller/runtime states outside the contract are untested.
// original: 0x00B2CFE0 garage_candidate_accept


use lf_checker_rt::global;

const OBJECT_VTABLE_OFFSET: usize = 0x00;
const OBJECT_MODEL_ID_OFFSET: usize = 0x2e;
const OBJECT_FLAGS_OFFSET: usize = 0x0f1f;
const OBJECT_REJECTION_STATE_OFFSET: usize = 0x1304;
const VTABLE_CANDIDATE_SLOT_OFFSET: usize = 0x178;
const CANDIDATE_DISQUALIFY_BIT: u8 = 0x20;

const GLOBAL_MODE_FILE_VA: u32 = 0x011d_6fd4;
const GLOBAL_MODEL_FILTER_A_FILE_VA: u32 = 0x012f_a308;
const GLOBAL_MODEL_FILTER_B_FILE_VA: u32 = 0x012f_a47c;
const GLOBAL_MODEL_FILTER_C_FILE_VA: u32 = 0x012f_a290;
const GLOBAL_MODEL_FILTER_D_FILE_VA: u32 = 0x012f_a008;

#[inline(always)]
unsafe fn read_at<T: Copy>(base: *const u8, offset: usize) -> T {
    unsafe { base.add(offset).cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn read_global(file_va: u32) -> i32 {
    unsafe { global::<i32>(file_va).read() }
}

/// Returns whether a candidate object passes its virtual eligibility check,
/// object-state gate and four global model filters. The first cdecl argument
/// points to an object with a vtable pointer at byte 0, flags at byte 0xF1F,
/// a signed model id at byte 0x2E and a rejection state at byte 0x1304.
/// The second cdecl argument is unused. The virtual method at vtable byte
/// offset 0x178 receives the candidate as this and returns its answer in AL.
/// Global model filters are read through relocated file VAs.
#[inline(always)]
unsafe fn candidate_accept(candidate: *const u8, _context: u32) -> u8 {
    let flags = unsafe { read_at::<u8>(candidate, OBJECT_FLAGS_OFFSET) };
    if flags & CANDIDATE_DISQUALIFY_BIT != 0 {
        return 0;
    }

    let vtable = unsafe { read_at::<*const u8>(candidate, OBJECT_VTABLE_OFFSET) };
    let method = unsafe { read_at::<*const ()>(vtable, VTABLE_CANDIDATE_SLOT_OFFSET) };
    let virtual_check: extern "thiscall" fn(*const u8) -> u8 =
        unsafe { core::mem::transmute(method) };
    let virtual_rejects = virtual_check(candidate) != 0;
    if virtual_rejects {
        return 0;
    }

    let rejection_state = unsafe { read_at::<u32>(candidate, OBJECT_REJECTION_STATE_OFFSET) };
    if rejection_state == 4 {
        return 0;
    }

    let model_id = unsafe { read_at::<i16>(candidate, OBJECT_MODEL_ID_OFFSET) } as i32;
    if model_id == unsafe { read_global(GLOBAL_MODEL_FILTER_A_FILE_VA) }
        || model_id == unsafe { read_global(GLOBAL_MODEL_FILTER_B_FILE_VA) }
        || model_id == unsafe { read_global(GLOBAL_MODEL_FILTER_C_FILE_VA) }
    {
        return 0;
    }

    let mode = unsafe { read_global(GLOBAL_MODE_FILE_VA) };
    if mode == 2 && model_id == unsafe { read_global(GLOBAL_MODEL_FILTER_D_FILE_VA) } {
        return 0;
    }

    1
}

lf_checker_rt::export!(cdecl, rw_00b2cfe0(candidate: *const u8, context: u32) -> u8 {
    unsafe { candidate_accept(candidate, context) }
});
