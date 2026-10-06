// original: 0x00C4D640 task_rows_release (proposed)

/// Release every entry of a task-row table and deregister its name tag.
///
/// `this` points to the table: a header pointer at `+0x00`, a signed row
/// count at `+0x08`, and rows of 0x68 bytes from `+0x00` (first slots at `+0x0c`). Each row holds
/// eight reference-counted slots at `+0x0c/+0x10/+0x14/+0x18` and
/// `+0x38/+0x3c/+0x40/+0x44`; the slots at `+0x10/+0x18/+0x3c/+0x44` are
/// skipped when they equal the shared default object (global
/// `0x17ED954`), the others when null. A kept slot whose 16-bit count at
/// `+0x0a` is non-zero is decremented; when the count reaches zero and the
/// flag byte at `+0x08` is 2 or 4, the object's slot-0 method is invoked
/// (thiscall, argument 1). After the rows the header pointer is released
/// the same way (null skips it).
///
/// Finally the table's name tag is looked up (callee 1); unless the lookup
/// reports -1 (missing) the handle is resolved (callee 2) and, when that is
/// non-null, deregistered (callee 3). All three are cdecl with one argument.
///
/// The row count is compared SIGNED: zero or negative counts skip the loop.
/// Original: 0x00C4D640 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00c4d640(this: u32) -> u32 {
    unsafe {
        const HEADER_PTR: u32 = 0x00;
        const ROW_COUNT: u32 = 0x08;
        const ROW_STRIDE: u32 = 0x68;
        const REF_FLAGS: u32 = 0x08;
        const REF_COUNT: u32 = 0x0a;
        const DESTROY_A: u8 = 2;
        const DESTROY_B: u8 = 4;
        const DEFAULT_OBJECT: u32 = 0x17ED954;
        const NAME_TAG: u32 = 0xEC9D08;
        const MISSING: u32 = 0xFFFF_FFFF;
        const LOOKUP: u32 = 1;
        const RESOLVE: u32 = 2;
        const DEREGISTER: u32 = 3;

        /// Decrement one slot's object, destroying it through slot 0 when
        /// the count reaches zero with a destroying flag.
        unsafe fn release(obj: u32) {
            unsafe {
                let count = ((obj + REF_COUNT) as *const u16).read_unaligned();
                if count == 0 {
                    return;
                }
                let left = count.wrapping_sub(1);
                ((obj + REF_COUNT) as *mut u16).write_unaligned(left);
                let flags = ((obj + REF_FLAGS) as *const u8).read();
                if left == 0 && (flags == DESTROY_A || flags == DESTROY_B) {
                    let table = (obj as *const u32).read_unaligned();
                    let slot0 = (table as *const u32).read_unaligned();
                    let destroy: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(slot0 as usize);
                    destroy(obj, 1);
                }
            }
        }

        #[inline(always)]
        unsafe fn slot(row: u32, off: u32) -> u32 {
            unsafe { ((row + off) as *const u32).read_unaligned() }
        }

        let shared = (lf_checker_rt::global::<u32>(DEFAULT_OBJECT) as *const u32)
            .read_unaligned();
        let n = ((this + ROW_COUNT) as *const i32).read_unaligned();
        if n > 0 {
            let mut i: i32 = 0;
            loop {
                let row = this
                    .wrapping_add((i as u32).wrapping_mul(ROW_STRIDE));
                let p0 = slot(row, 0x0c);
                if p0 != 0 {
                    release(p0);
                }
                let p1 = slot(row, 0x10);
                if p1 != shared {
                    release(p1);
                }
                let p2 = slot(row, 0x14);
                if p2 != 0 {
                    release(p2);
                }
                let p3 = slot(row, 0x18);
                if p3 != shared {
                    release(p3);
                }
                let p4 = slot(row, 0x38);
                if p4 != 0 {
                    release(p4);
                }
                let p5 = slot(row, 0x3c);
                if p5 != shared {
                    release(p5);
                }
                let p6 = slot(row, 0x40);
                if p6 != 0 {
                    release(p6);
                }
                let p7 = slot(row, 0x44);
                if p7 != shared {
                    release(p7);
                }
                i += 1;
                if i >= n {
                    break;
                }
            }
        }
        let head = ((this + HEADER_PTR) as *const u32).read_unaligned();
        if head != 0 {
            release(head);
        }
        let tag = lf_checker_rt::relocated(NAME_TAG);
        let handle: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, tag);
        if handle == MISSING {
            return 0;
        }
        let resolved: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, handle);
        if resolved == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(DEREGISTER, u32, handle);
        0
    }
});
