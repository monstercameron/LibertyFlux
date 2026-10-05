// original: 0x00e5c090 rotate_head_save_old_e5c090

/// Rewrite of a global-slot rotation: saves the current head word
/// into this stub's private slot, installs the stub's constant as the
/// new head, and returns the previous head.
///
/// Original shape: `(an instruction of the original); (an instruction of the original); (an instruction of the original); ret`.
export!(cdecl, rw_00e5c090() -> u32 {
    unsafe {
        let head = global::<u32>(0x017AD16C);
        let old = *head;
        *(global::<u32>(0x0110DBAC) as *mut u32) = old;
        *head = relocated(0x0110DB9C);
        old
    }
});
