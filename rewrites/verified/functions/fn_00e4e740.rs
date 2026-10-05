// original: 0x00e4e740 delete_clip_and_reconcile
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Delete a gallery clip file and its tag sidecar, then reconcile the
/// viewer's item list (original 0x00E4E740).
///
/// Builds a path from a global directory prefix plus the per-item suffix at
/// `this+0x350+0x1F8`, deletes that file, swaps the last three characters
/// for the `tag` extension and deletes the sidecar too. Then it retires the
/// `+0x350` object through a helper-created replacement, and either takes a
/// short release path (replacement quiet but secondary check exactly 1) or
/// sweeps the counted item table under `+0x32C`, refreshing each entry whose
/// status query is below the `+0x30C` threshold, accumulating a second status
/// sweep into a sum that gates one final notification when it does not exceed
/// `+0x354`, and finally marks the `+0x32C` object dirty (`+0x218 = 1`) and
/// notifies it twice. Does nothing when `+0x350` is null. Returns nothing.
///
/// The overlong-path branch (>= 0x203 chars) is kept exact but unreachable in
/// trials: any such path already overwrote the original's security cookie, so
/// its behaviour is frame-dependent.
export!(thiscall, rw_00e4e740(this: u32) -> () {
    const SUFFIX_OFF: u32 = 0x1F8;
    const PREFIX: u32 = 0x01168DD8;
    const TAG_WORD: u32 = 0x00F19B64;
    const CLIPS_DIR: u32 = 0x00F19C68;
    const FACTORY: u32 = 0x01981A4C;
    const PATH_MAX: usize = 0x200;
    unsafe {
        let words = this as *const u32;
        let obj = words.add(0x350 / 4).read();
        if obj == 0 {
            return;
        }
        callee_cdecl!(1, u32, relocated(CLIPS_DIR), 0);
        // path = prefix + suffix; both short NUL-terminated strings.
        let mut path = [0u8; PATH_MAX];
        let prefix = relocated(PREFIX) as *const u8;
        let mut pre_len = 0usize;
        loop {
            let b = prefix.add(pre_len).read();
            path[pre_len] = b;
            pre_len += 1;
            if b == 0 {
                break;
            }
        }
        let pre_len = pre_len - 1;
        let suffix = (obj.wrapping_add(SUFFIX_OFF)) as *const u8;
        let mut suf_len = 0usize;
        while suffix.add(suf_len).read() != 0 {
            suf_len += 1;
        }
        let mut i = 0usize;
        while i <= suf_len {
            path[pre_len + i] = suffix.add(i).read();
            i += 1;
        }
        delete_file(path.as_ptr() as u32);
        let full_len = (pre_len + suf_len) as u32;
        let stem_len = full_len.wrapping_sub(3);
        if stem_len >= 0x200 {
            // Original jumps to a fatal-error stub here; unreachable without
            // smashing its stack cookie (see doc comment).
            panic!("gallery clip path too long");
        }
        path[stem_len as usize] = 0;
        let tag = (relocated(TAG_WORD) as *const u32).read();
        ((path.as_mut_ptr() as u32).wrapping_add(stem_len) as *mut u32).write(tag);
        delete_file(path.as_ptr() as u32);
        // Retire the +0x350 object through its replacement.
        let secondary = words.add(0x32C / 4).read();
        let token = vcall0(obj, 0x54);
        let fresh = callee_thiscall!(4, u32, relocated(FACTORY), token);
        if words.add(0x350 / 4).read() != 0 {
            vcall1(obj, 0x08, 1);
        }
        (this as *mut u32).add(0x350 / 4).write(0);
        vcall0(fresh, 0x238);
        vcall1(fresh, 0x218, 0);
        if vcall0(fresh, 0x1D4) != 0 {
            sweep_items(this, secondary, fresh);
            return;
        }
        if vcall0(secondary, 0x1D4) != 1 {
            sweep_items(this, secondary, fresh);
            return;
        }
        vcall1(fresh, 0x08, 1);
        callee_thiscall!(11, u32, secondary);
    }
});

/// Main item-table sweep plus the final notification sequence.
fn sweep_items(this: u32, secondary: u32, fresh: u32) {
    unsafe {
        let list = ((secondary.wrapping_add(0x1E0)) as *const u32).read();
        let first = vcall0(list, 0x1D0);
        let again = vcall0(list, 0x1D0);
        let first_base = (first as *const u32).read();
        if first_base == list_end(again) {
            finish_sweep(this, secondary);
            return;
        }
        let mut cursor = first_base.wrapping_add(4);
        loop {
            let prev = (cursor.wrapping_sub(4) as *const u32).read();
            if vcall0(prev, 0x1D4) == 0 {
                if vcall0(secondary, 0x1D4) > 1 {
                    callee_thiscall!(24, u32, secondary);
                }
                vcall1(prev, 0x08, 1);
                break;
            }
            let threshold = ((this.wrapping_add(0x30C)) as *const u32).read();
            if vcall0(prev, 0x1D4) >= threshold {
                cursor = cursor.wrapping_add(4);
                if cursor.wrapping_sub(4) == list_end(vcall0(list, 0x1D0)) {
                    break;
                }
                continue;
            }
            let current = (cursor as *const u32).read();
            if cursor != list_end(vcall0(list, 0x1D0)) {
                if vcall0(current, 0x1D4) != 0 {
                    let session = vcall1(current, 0x1E0, 0);
                    let cookie = vcall0(session, 0x4C);
                    vcall1(prev, 0x1D8, cookie);
                    let stamp = vcall0(prev, 0x4C);
                    vcall1(session, 0x58, stamp);
                    if current == fresh {
                        vcall2(current, 0x22C, 0, 1);
                    }
                    let ticket = vcall1(current, 0x1D0, 0);
                    callee_thiscall!(23, u32, ticket);
                    vcall0(current, 0x238);
                    vcall0(prev, 0x238);
                }
            }
            if cursor != list_end(vcall0(list, 0x1D0)) {
                if vcall0(current, 0x1D4) == 0 {
                    vcall1(current, 0x08, 1);
                }
            }
            cursor = cursor.wrapping_add(4);
            if cursor.wrapping_sub(4) != list_end(vcall0(list, 0x1D0)) {
                continue;
            }
            break;
        }
        finish_sweep(this, secondary);
    }
}

/// Second status sweep with the accumulating sum, the gated notification and
/// the final dirty-mark plus double notification of the secondary object.
fn finish_sweep(this: u32, secondary: u32) {
    unsafe {
        let list = ((secondary.wrapping_add(0x1E0)) as *const u32).read();
        let head = vcall0(list, 0x1D0);
        let head_base = (head as *const u32).read();
        let mut sum: u32 = 0;
        let mut cursor = head_base;
        if head_base != list_end(vcall0(list, 0x1D0)) {
            loop {
                let item = (cursor as *const u32).read();
                sum = sum.wrapping_add(vcall0(item, 0x1D4));
                cursor = cursor.wrapping_add(4);
                if cursor == list_end(vcall0(list, 0x1D0)) {
                    break;
                }
            }
        }
        let budget = ((this.wrapping_add(0x354)) as *const u32).read();
        if sum <= budget {
            let secondary2 = ((this.wrapping_add(0x32C)) as *const u32).read();
            let target = ((secondary2.wrapping_add(0x1EC)) as *const u32).read();
            vcall1(target, 0x120, 0);
        }
        callee_thiscall!(11, u32, secondary);
        (secondary.wrapping_add(0x218) as *mut u8).write(1);
        vcall1(secondary, 0x18, 1);
        callee_thiscall!(27, u32, secondary, 0, 1);
    }
}

/// End pointer of a counted table struct: base word plus u16 count words.
unsafe fn list_end(table: u32) -> u32 {
    let base = (table as *const u32).read();
    let count = ((table.wrapping_add(4)) as *const u16).read() as u32;
    base.wrapping_add(count.wrapping_mul(4))
}

/// File deletion through the import slot, exactly like the original's
/// `(an instruction of the original)`: both sides land on the checker's recorder stub.
unsafe fn delete_file(path: u32) -> u32 {
    const DELETE_FILE_SLOT: u32 = 0x00E73268;
    let addr = global::<u32>(DELETE_FILE_SLOT).read();
    let f: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(path)
}

/// Virtual call with no stack arguments: object in ECX, callee cleans up.
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj)
}

/// Virtual call with one stack argument.
unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj, a0)
}

/// Virtual call with two stack arguments.
unsafe fn vcall2(obj: u32, slot: u32, a0: u32, a1: u32) -> u32 {
    let vtable = (obj as *const u32).read();
    let addr = ((vtable.wrapping_add(slot)) as *const u32).read();
    let f: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj, a0, a1)
}
