// original: 0x00ca9b40 CEventHandler::vf62
/// Handle one event for an event handler: clear the handler's state word,
/// then either construct a response object for an ordinary event id or walk
/// the handler's node list for the special event id.
///
/// `this` is the handler: word at `+0x04` points to the owner, whose word at
/// `+0x224` is the inner object. `ev` is the event, whose id word at `+0x0c`
/// selects the path. The second and third stack arguments are not read.
/// Returns whatever the constructing callee returned, or zero when
/// allocation failed. (thiscall: `this` in ecx, three stack words.)
///
/// The inner object's word at `+0x264` is always cleared first, and the
/// chosen response object (or zero) is stored at `this+0x0c` on every path.
///
/// Ordinary path (`id != 0x3ae`): allocate an object through the allocator
/// callee (id 1, fed the shared global); on allocation failure store zero
/// and return zero. Otherwise run the eight-argument constructor callee
/// (id 2: flags `0, 1`, factor `4.0`, kind `0x19b`, descriptor table,
/// `0`, factor `1.0`, `0`), plant the response vtable, store the object and
/// return the constructor's answer.
///
/// Special path (`id == 0x3ae`): walk the singly linked node list at
/// `inner+0x2e0` (each node: tag at `+0x04`, key bits at `+0x08`, next at
/// `+0x0c`). The walk bails out the moment a node's 3-bit key
/// (`(word >> 1) & 7`) rises above the previous one. A node whose tag is
/// `0x10c` selects the two-argument constructor (id 4: `0x100, 0`);
/// reaching the end of the list (or an empty list, or a rising key) selects
/// the zero-argument constructor (id 3). Both take a freshly allocated
/// object; a failed allocation stores zero and returns zero.
lf_checker_rt::export!(thiscall, rw_00ca9b40(this: u32, ev: u32, _a1: u32, _a2: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x04;
        const INNER: u32 = 0x224;
        const STATE: u32 = 0x264;
        const RESPONSE: u32 = 0x0c;
        const EVENT_ID: u32 = 0x0c;
        const SPECIAL_ID: u32 = 0x3ae;
        const NODE_LIST: u32 = 0x2e0;
        const NODE_TAG: u32 = 0x04;
        const NODE_KEY: u32 = 0x08;
        const NODE_NEXT: u32 = 0x0c;
        const WANTED_TAG: u32 = 0x10c;
        const CTOR_KIND: u32 = 0x19b;
        const CTOR_SIZE: u32 = 0x100;
        const ONE: u32 = 0x3f80_0000;
        const FOUR: u32 = 0x4080_0000;
        const DESCRIPTOR_TABLE: u32 = 0x00ed_7b48;
        const RESPONSE_VTABLE: u32 = 0x00ed_7af4;
        const SHARED_GLOBAL: u32 = 0x0167_e2a0;
        const ALLOC: u32 = 1;
        const CTOR8: u32 = 2;
        const CTOR0: u32 = 3;
        const CTOR2: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let inner = rd32(rd32(this + OWNER) + INNER);
        wr32(inner + STATE, 0);
        let global = rd32(lf_checker_rt::relocated(SHARED_GLOBAL));

        if rd32(ev + EVENT_ID) != SPECIAL_ID {
            let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
            if obj == 0 {
                wr32(this + RESPONSE, 0);
                return 0;
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(
                CTOR8, u32, obj, 0, 1, FOUR, CTOR_KIND,
                lf_checker_rt::relocated(DESCRIPTOR_TABLE), 0, ONE, 0
            );
            wr32(obj, lf_checker_rt::relocated(RESPONSE_VTABLE));
            wr32(this + RESPONSE, obj);
            return r;
        }

        let mut node = rd32(inner + NODE_LIST);
        let mut want_tagged = false;
        if node != 0 {
            let mut prev = (rd32(node + NODE_KEY) >> 1) & 7;
            loop {
                let cur = (rd32(node + NODE_KEY) >> 1) & 7;
                if prev < cur {
                    break;
                }
                prev = cur;
                if rd32(node + NODE_TAG) == WANTED_TAG {
                    want_tagged = true;
                    break;
                }
                node = rd32(node + NODE_NEXT);
                if node == 0 {
                    break;
                }
            }
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, global);
        if obj == 0 {
            wr32(this + RESPONSE, 0);
            return 0;
        }
        if want_tagged {
            let r: u32 = lf_checker_rt::callee_thiscall!(CTOR2, u32, obj, CTOR_SIZE, 0);
            wr32(this + RESPONSE, r);
            r
        } else {
            let r: u32 = lf_checker_rt::callee_thiscall!(CTOR0, u32, obj);
            wr32(this + RESPONSE, r);
            r
        }
    }
});
