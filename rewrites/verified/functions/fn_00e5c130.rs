// original: 0x00e5c130 rotate_head_save_old_e5c130

/// Rewrite of a global-slot rotation: saves the current head word
/// into this stub's private slot, installs the stub's constant as the
/// new head, and returns the previous head.
///
/// Original shape: `(an instruction of the original); (an instruction of the original); (an instruction of the original); ret`.
export!(cdecl, rw_00e5c130() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD1B8);
        let old = *head;
        *(global::<u32>(0x0110DD84) as *mut u32) = old;
        *head = relocated(0x0110DD78);
        old
    }
});
