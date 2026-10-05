// original: 0x0065c060 lazy_init_singleton_0065c060 (proposed)

/// Lazily create and register one engine subsystem descriptor (singleton).
///
/// If the slot `0x018B79CC` already holds a descriptor, returns at once. Otherwise
/// allocates a `0x40-byte object through the thread allocator reached from TLS
/// slot 0 (`[slot + 8]` -> vtable -> slot `+8`, called with (`0x40, 0x10, 0)
/// with the allocator in ECX), runs the constructor callee on it, publishes it
/// to the slot
/// and fills the descriptor: name id `0xF9AFE8` at `+0x04`, kind `0x70` at `+0x10`,
/// handler table `0x65DE70` at `+0x24`, trailer `0x61B460` at `+0x2c`, zeroes at
/// `+0x20`/`+0x28`. Then writes 3 small constants into the shared offset
/// table, links the parent object from `0x018B74C4` at `+0x08` (building it first
/// when the slot is empty), folds bits from the parent's word at `+0x1c` into
/// the descriptor's (`masked = parent & 0xffc0`, `mix = (obj ^ masked) & 0xffff`,
/// `obj ^= mix`, `[+0x1e] = masked >> 16`), and finishes with the register
/// callee (table `0x10FDDBC`, thiscall) and the finalise callee, which takes the
/// registry object at `0x01BB5520+0x18` in ECX and pointers to two stack slots holding
/// this slot's address and the name id. No stack arguments; returns nothing
/// meaningful (checked with `ret: none`).
///
/// Original: 0x0065c060 (cdecl, no arguments). All address immediates in the
/// original carry relocations, so the rewrite resolves them through
/// `relocated`/`global` for the worker's mapping.
lf_checker_rt::export!(cdecl, rw_0065c060() -> u32 {
    unsafe {
        const SELF_SLOT: u32 = 0x018B79CC;
        const PARENT_SLOT: u32 = 0x018B74C4;
        const REGISTRY_SLOT: u32 = 0x01BB5520;
        const DESCR_NAME: u32 = 0xF9AFE8;
        const DESCR_KIND: u32 = 0x70;
        const DESCR_OPS: u32 = 0x65DE70;
        const DESCR_TRAILER: u32 = 0x61B460;
        const PUSHED_TABLE: u32 = 0x10FDDBC;
        const TABLE_WRITES: [(u32, u32); 3] = [(0x10FE064, 0x14), (0x10FDCC0, 0x18), (0x10FE01C, 0x6C)];
        const ALLOC_CALLEE: u32 = 1;
        const CTOR_CALLEE: u32 = 2;
        const ENSURE_CALLEE: u32 = 3;
        const REGISTER_CALLEE: u32 = 4;
        const FINISH_CALLEE: u32 = 5;
        const ALLOC_SIZE: u32 = 0x40;
        const ALLOC_ALIGN: u32 = 0x10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wg32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::relocated(va) as *mut u32).write_unaligned(v) }
        }

        use lf_checker_rt::{callee_addr, relocated, tls_slot};

        if g32(SELF_SLOT) != 0 {
            return 0;
        }
        let thread_data = tls_slot(0);
        let allocator = rd32(thread_data + 8);
        let vtable = rd32(allocator);
        let alloc_fn: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + 8) as usize);
        let obj = alloc_fn(allocator, ALLOC_SIZE, ALLOC_ALIGN, 0);
        let obj = if obj != 0 {
            lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, obj)
        } else {
            0
        };
        wg32(SELF_SLOT, obj);
        wr32(obj + 0x04, DESCR_NAME);
        wr32(g32(SELF_SLOT) + 0x10, DESCR_KIND);
        wr32(g32(SELF_SLOT) + 0x20, 0);
        wr32(g32(SELF_SLOT) + 0x24, relocated(DESCR_OPS));
        wr32(g32(SELF_SLOT) + 0x28, 0);
        wr32(g32(SELF_SLOT) + 0x2c, relocated(DESCR_TRAILER));
        let mut parent = g32(PARENT_SLOT);
        let mut ti = 0;
        while ti < TABLE_WRITES.len() {
            wg32(TABLE_WRITES[ti].0, TABLE_WRITES[ti].1);
            ti += 1;
        }
        if parent == 0 {
            lf_checker_rt::callee_cdecl!(ENSURE_CALLEE, u32,);
            parent = g32(PARENT_SLOT);
        }
        let g = g32(SELF_SLOT);
        wr32(g + 0x08, parent);
        wr32(g + 0x0c, 0);
        let parent_bits = rd32(parent + 0x1c);
        let obj_bits = rd32(g + 0x1c);
        let masked = parent_bits & 0xffc0;
        let mixed = (obj_bits ^ masked) & 0xffff;
        wr32(g + 0x1c, obj_bits ^ mixed);
        wr16(g + 0x1e, (masked >> 16) as u16);
        lf_checker_rt::callee_thiscall!(
            REGISTER_CALLEE,
            u32,
            g32(SELF_SLOT),
            relocated(PUSHED_TABLE)
        );
        let g = g32(SELF_SLOT);
        let registry = g32(REGISTRY_SLOT);
        let name_out = rd32(g + 0x04);
        let mut slot_for_table: u32 = relocated(SELF_SLOT);
        let mut slot_for_name: u32 = name_out;
        let finish: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(FINISH_CALLEE) as usize);
        finish(
            registry + 0x18,
            &mut slot_for_name as *mut u32 as u32,
            &mut slot_for_table as *mut u32 as u32,
        );
        0
    }
});
