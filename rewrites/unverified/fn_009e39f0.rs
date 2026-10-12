// original: 0x009E39F0 tbd
/// Stores the dispatch mode and any nonnegative value, then calls the
/// attachment worker with mode flags selected by the state record and its
/// indexed runtime entry. The routine returns no value.
#![allow(unsafe_code)]

const ATTACH_TABLE_FILE_VA: u32 = 0x0129_5CD8;
const TABLE_FLAGS_OFFSET: u32 = 0x94;
const MODE_FLAGS_MASK: u32 = 0x3C0;
const MODE_FLAGS_VALUE: u32 = 0x80;
const EXTRA_FLAG_A: u32 = 0x800;
const EXTRA_FLAG_B: u32 = 0x1000;

#[inline(always)]
unsafe fn read_word(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

#[inline(always)]
unsafe fn read_i16(address: u32) -> i16 {
    (address as *const i16).read_unaligned()
}

#[inline(always)]
unsafe fn write_word(address: u32, value: u32) {
    (address as *mut u32).write_unaligned(value);
}

#[inline(always)]
fn call_attachment_worker(id: u32, this: u32, state: u32, value: u32) {
    let _ = lf_checker_rt::callee_thiscall!(
        id,
        u32,
        this,
        state,
        u32::MAX,
        value,
        0,
        0,
    );
}

lf_checker_rt::export!(thiscall, fn_009e39f0(this: u32, state: u32, _unused: u32, dispatch_mode: u32, dispatch_value: u32) -> () {
    unsafe {
        write_word(this.wrapping_add(0xD50), dispatch_mode);
        if (dispatch_value as i32) >= 0 {
            write_word(this.wrapping_add(0xD54), dispatch_value);
        }

        let state_flags = read_word(state.wrapping_add(0x28)) & MODE_FLAGS_MASK;
        if state_flags != MODE_FLAGS_VALUE {
            call_attachment_worker(3, this, state, dispatch_value);
            return;
        }

        if read_word(state.wrapping_add(0x1300)) == 1 {
            let object_flags = read_word(this.wrapping_add(0x264)) & 6;
            let masked_value = dispatch_value | EXTRA_FLAG_A;
            if object_flags == 2 {
                call_attachment_worker(3, this, state, masked_value);
            } else {
                call_attachment_worker(1, this, state, masked_value | EXTRA_FLAG_B);
            }
            return;
        }

        let state_kind = read_word(state.wrapping_add(0x1304));
        let mode_is_special = dispatch_mode == 1 || dispatch_mode == 3;
        if state_kind == 4 && !mode_is_special {
            call_attachment_worker(2, this, state, dispatch_value | EXTRA_FLAG_A);
            return;
        }

        let entry_index = unsafe { read_i16(state.wrapping_add(0x2E)) } as i32;
        let entry_file_va = ATTACH_TABLE_FILE_VA
            .wrapping_add((entry_index as u32).wrapping_mul(4));
        let entry_address = unsafe { lf_checker_rt::global::<u32>(entry_file_va).read_unaligned() };
        let entry_flags = unsafe { read_word(entry_address.wrapping_add(TABLE_FLAGS_OFFSET)) };
        let table_gate = entry_flags.wrapping_shr(5) & 1 != 0;

        if table_gate && !mode_is_special {
            call_attachment_worker(2, this, state, dispatch_value | EXTRA_FLAG_A);
        } else if state_kind == 2 {
            call_attachment_worker(2, this, state, dispatch_value | EXTRA_FLAG_A);
        } else {
            call_attachment_worker(3, this, state, dispatch_value);
        }
    }
});
