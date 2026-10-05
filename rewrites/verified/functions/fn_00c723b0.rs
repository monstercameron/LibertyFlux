// original: 0x00c723b0 ped_task_spatial_query_alloc (proposed)

/// Allocate a task object and link it against matching spatial results.
///
/// Takes five stack arguments: two point objects `a` and `b` (three floats
/// each at `+0`, `+4`, `+8`) and three integers. When the spatial index
/// global is null it is created: a helper (intercepted thiscall) builds an
/// index over a fixed 3000-unit box (laid out as -3000, +3000, +3000, -3000)
/// and the index builder (intercepted thiscall of the box and 5) makes the
/// index, which is stored to the global (a null helper result stores 0). A
/// fresh 0x50-byte block (intercepted cdecl allocator) is then constructed
/// by the task builder (intercepted thiscall of the five arguments); a null
/// block leaves the new task null. The midpoints of the two points are
/// queried against the index (intercepted thiscall of the midpoint triple
/// and an out-slot) together with the four raw endpoint words kept aside.
/// Each result node holds an object at `+0` and the next node at `+4`; the
/// new task itself is skipped, and the first node whose six floats at `+0`,
/// `+4`, `+8`, `+0x10`, `+0x14`, `+0x18` all compare equal (the `ucomiss;
/// lahf; (an instruction of the original)` idiom: ordered-equal passes, anything else fails)
/// swaps its word at `+0x4C` with the new task's and ends the routine after
/// releasing the list. With no match, the new task and the endpoint words
/// are handed to the inserter (intercepted thiscall) before the release.
///
/// Original: 0x00C723B0 (cdecl, five stack arguments; no defined return).
lf_checker_rt::export!(cdecl, rw_00C723B0(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    unsafe {
        const INDEX_GLOBAL: u32 = 0x16DD680;
        const HELPER_GLOBAL: u32 = 0x12FB254;
        const HALF: u32 = 0xFE8830; // 0.5
        const NEG_RANGE: u32 = 0xC53B8000; // -3000.0
        const POS_RANGE: u32 = 0x453B8000; // 3000.0
        const MAKE_HELPER: u32 = 1;
        const MAKE_INDEX: u32 = 2;
        const ALLOC: u32 = 3;
        const BUILD: u32 = 4;
        const QUERY: u32 = 5;
        const INSERT: u32 = 6;
        const RELEASE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(p: u32) -> f32 {
            unsafe { f32::from_bits(rd32(p)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn set_g32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(va) as *mut u32).write(v) }
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }

        if g32(INDEX_GLOBAL) == 0 {
            let helper: u32 =
                lf_checker_rt::callee_thiscall!(MAKE_HELPER, u32, g32(HELPER_GLOBAL));
            if helper == 0 {
                set_g32(INDEX_GLOBAL, 0);
            } else {
                let bounds = [NEG_RANGE, POS_RANGE, POS_RANGE, NEG_RANGE];
                let index: u32 = lf_checker_rt::callee_thiscall!(
                    MAKE_INDEX,
                    u32,
                    helper,
                    bounds.as_ptr() as u32,
                    5
                );
                set_g32(INDEX_GLOBAL, index);
            }
        }
        let block: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, 0x50);
        let task: u32 = if block == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BUILD, u32, block, a, b, c, d, e)
        };
        let ax = rdf(a);
        let ay = rdf(a + 4);
        let az = rdf(a + 8);
        let bx = rdf(b);
        let by = rdf(b + 4);
        let bz = rdf(b + 8);
        let half = f32::from_bits(g32(HALF));
        let ends = [ax.to_bits(), by.to_bits(), bx.to_bits(), ay.to_bits()];
        let mid = [
            mul(add(bx, ax), half).to_bits(),
            mul(add(by, ay), half).to_bits(),
            mul(add(bz, az), half).to_bits(),
        ];
        let mut head: u32 = 0;
        let head_ptr = (&mut head as *mut u32) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            QUERY,
            u32,
            g32(INDEX_GLOBAL),
            mid.as_ptr() as u32,
            head_ptr
        );
        let mut node = head;
        while node != 0 {
            let obj = rd32(node);
            if obj != task {
                let mut same = true;
                for off in [0u32, 4, 8, 0x10, 0x14, 0x18] {
                    if rdf(obj + off) != rdf(task + off) {
                        same = false;
                        break;
                    }
                }
                if same {
                    let tmp = rd32(obj + 0x4C);
                    ((task + 0x4C) as *mut u32).write_unaligned(tmp);
                    ((obj + 0x4C) as *mut u32).write_unaligned(task);
                    let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, head_ptr);
                    return 0;
                }
            }
            node = rd32(node + 4);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            INSERT, u32, g32(INDEX_GLOBAL), task, ends.as_ptr() as u32
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, head_ptr);
        0
    }
});
