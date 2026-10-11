// original: 0x00B38C20 ped_task_callback_dispatch_stage

/// Resolves one callback key by comparing it with the game's registered
/// callback slots. Two keys return stored values directly; groups of keys
/// tail-dispatch to one of three callback helpers with the registry object in
/// ECX. Unmatched keys return -1, except for the final special key, which
/// returns its associated stored value. The entry is cdecl with one stack
/// word. The comparison is exact unsigned word equality.
lf_checker_rt::export!(cdecl, rw_00b38c20(key: u32) -> u32 {
    unsafe { resolve_callback_key_00b38c20(key, false) }
});

const KEY_INITIALIZE_00B38C20: u32 = 0x012F_A308;
const KEY_REFRESH_00B38C20: u32 = 0x012F_A47C;
const KEY_C68340_A_00B38C20: u32 = 0x012F_9FB4;
const KEY_C68340_B_00B38C20: u32 = 0x012F_9FF0;
const KEY_C68340_C_00B38C20: u32 = 0x012F_A62C;
const KEY_C68340_D_00B38C20: u32 = 0x012F_9F24;
const KEY_C68340_E_00B38C20: u32 = 0x012F_A044;
const KEY_C68340_F_00B38C20: u32 = 0x012F_A068;
const KEY_C68340_G_00B38C20: u32 = 0x012F_9E34;
const KEY_C68340_H_00B38C20: u32 = 0x012F_A284;
const KEY_C68790_A_00B38C20: u32 = 0x012F_A314;
const KEY_C68790_B_00B38C20: u32 = 0x012F_A494;
const KEY_C68790_C_00B38C20: u32 = 0x012F_A3A4;
const KEY_C68450_00B38C20: u32 = 0x012F_9DBC;
const KEY_RETURN_C_00B38C20: u32 = 0x012F_9EA0;
const KEY_C68790_D_00B38C20: u32 = 0x012F_A428;
const KEY_C68790_E_00B38C20: u32 = 0x012F_A374;
const KEY_C68790_F_00B38C20: u32 = 0x012F_A56C;
const KEY_RETURN_D_00B38C20: u32 = 0x012F_A230;
const KEY_RETURN_E_00B38C20: u32 = 0x012F_9EAC;
const KEY_RETURN_F_00B38C20: u32 = 0x012F_A38C;
const RETURN_INITIALIZE_00B38C20: u32 = 0x012F_A164;
const RETURN_REFRESH_00B38C20: u32 = 0x012F_A128;
const RETURN_C_00B38C20: u32 = 0x012F_A140;
const RETURN_D_E_00B38C20: u32 = 0x012F_A0C8;
const RETURN_F_00B38C20: u32 = 0x012F_9E04;
const CALLBACK_REGISTRY_FILE_VA_00B38C20: u32 = 0x016D_CEB8;

#[inline(always)]
unsafe fn read_u32_00b38c20(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn global_u32_00b38c20(file_va: u32) -> u32 {
    unsafe { read_u32_00b38c20(lf_checker_rt::global::<u32>(file_va) as u32) }
}

unsafe fn resolve_callback_key_00b38c20(key: u32, mutant: bool) -> u32 {
    unsafe {
        if key == global_u32_00b38c20(KEY_INITIALIZE_00B38C20) {
            let value = global_u32_00b38c20(RETURN_INITIALIZE_00B38C20);
            return if mutant { value.wrapping_add(1) } else { value };
        }
        if key == global_u32_00b38c20(KEY_REFRESH_00B38C20) {
            return global_u32_00b38c20(RETURN_REFRESH_00B38C20);
        }
        if key == global_u32_00b38c20(KEY_C68340_A_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_B_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_C_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_D_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_E_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_F_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_G_00B38C20)
            || key == global_u32_00b38c20(KEY_C68340_H_00B38C20)
        {
            return lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CALLBACK_REGISTRY_FILE_VA_00B38C20));
        }
        if key == global_u32_00b38c20(KEY_C68790_A_00B38C20)
            || key == global_u32_00b38c20(KEY_C68790_B_00B38C20)
            || key == global_u32_00b38c20(KEY_C68790_C_00B38C20)
        {
            return lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CALLBACK_REGISTRY_FILE_VA_00B38C20));
        }
        if key == global_u32_00b38c20(KEY_C68450_00B38C20) {
            return lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(CALLBACK_REGISTRY_FILE_VA_00B38C20));
        }
        if key == global_u32_00b38c20(KEY_RETURN_C_00B38C20) {
            return global_u32_00b38c20(RETURN_C_00B38C20);
        }
        if key == global_u32_00b38c20(KEY_C68790_D_00B38C20)
            || key == global_u32_00b38c20(KEY_C68790_E_00B38C20)
            || key == global_u32_00b38c20(KEY_C68790_F_00B38C20)
        {
            return lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CALLBACK_REGISTRY_FILE_VA_00B38C20));
        }
        if key == global_u32_00b38c20(KEY_RETURN_D_00B38C20) {
            return global_u32_00b38c20(RETURN_D_E_00B38C20);
        }
        if key == global_u32_00b38c20(KEY_RETURN_E_00B38C20) {
            return global_u32_00b38c20(RETURN_D_E_00B38C20);
        }
        if key == global_u32_00b38c20(KEY_RETURN_F_00B38C20) {
            return global_u32_00b38c20(RETURN_F_00B38C20);
        }
        u32::MAX
    }
}
