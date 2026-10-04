// original: 0x00df7350 UIFileViewer::vf100
/// Refresh the file viewer when the focused clip's frame is on screen.
/// Resolves the current clip object and reads four measurements off each of
/// two related objects through their float-returning slots, combining each
/// pair with the global scale into a bound (two maxima, two minima). Two
/// further globals-derived limits are clamped into range, and unless all
/// four bounds strictly contain the limits the viewer is left alone.
/// Otherwise a highlight event is raised and the viewer is notified through
/// its slot 0x1C0. Does nothing when there is no current clip.
export!(thiscall, rw_00df7350(this: u32, arg0: u32) -> u32 {
    unsafe {
        /// Shared UI service the setup calls go through (file VA; the
        /// worker relocates the image, so derive the mapped address).
        const UI_SERVICE: u32 = 0x01981A4C;
        /// Event sink for the highlight event (file VA, relocated likewise).
        const EVENT_SINK: u32 = 0x01176888;
        let ui_service = relocated(UI_SERVICE);
        let event_sink = relocated(EVENT_SINK);
        /// Per-side measure slots, in call order.
        const SLOTS: [u32; 8] = [0xC8, 0xB8, 0xB8, 0xC8, 0xD0, 0xC0, 0xC0, 0xD0];
        /// Notify slot on the viewer itself.
        const NOTIFY_SLOT: u32 = 0x1C0;

        let side_a: u32 = callee_thiscall!(1, u32, ui_service, arg0);
        let _: u32 = callee_thiscall!(2, u32, this, 1);
        let tmp: u32 = callee_thiscall!(3, u32, ui_service, relocated(0x00F01BA0), 0);
        let _: u32 = callee_thiscall!(4, u32, tmp);
        let side_b: u32 = callee_thiscall!(5, u32, this);
        if side_b == 0 {
            return 0;
        }
        let scale = *(global::<f32>(0x00FE8830));
        let mut a = [0.0f32; 8];
        let mut b = [0.0f32; 8];
        let vt_a = *(side_a as *const u32);
        for (i, slot) in SLOTS.iter().enumerate() {
            let addr = *((vt_a.wrapping_add(*slot)) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(addr as usize);
            a[i] = f(side_a);
        }
        let vt_b = *(side_b as *const u32);
        for (i, slot) in SLOTS.iter().enumerate() {
            let addr = *((vt_b.wrapping_add(*slot)) as *const u32);
            let f: extern "thiscall" fn(u32) -> f32 =
                core::mem::transmute(addr as usize);
            b[i] = f(side_b);
        }
        // Each side's four bounds from its eight measures.
        let (aa, ab, ac, ad) = (
            a[0] - a[1] * scale,
            a[3] + a[2] * scale,
            a[4] - a[5] * scale,
            a[7] + a[6] * scale,
        );
        let (ba, bb, bc, bd) = (
            b[0] - b[1] * scale,
            b[3] + b[2] * scale,
            b[4] - b[5] * scale,
            b[7] + b[6] * scale,
        );
        // Union across sides: maxima of the first and third, minima of the
        // second and fourth. Each `if` mirrors a comiss+jbe pair, so an
        // unordered (NaN) comparison keeps the running value.
        let mut r_max_a = aa;
        if ba > r_max_a {
            r_max_a = ba;
        }
        let mut r_min_b = ab;
        if r_min_b > bb {
            r_min_b = bb;
        }
        let mut r_max_c = ac;
        if bc > r_max_c {
            r_max_c = bc;
        }
        let mut r_min_d = ad;
        if r_min_d > bd {
            r_min_d = bd;
        }
        let hi = *(global::<f32>(0x00FE88E8));
        let mut lim0 =
            (*(global::<i32>(0x018B7A8C)) as f32) * *(global::<f32>(0x017ACCF0));
        if 0.0 > lim0 {
            lim0 = 0.0;
        }
        if lim0 > hi {
            lim0 = hi;
        }
        let raw1 =
            (*(global::<i32>(0x018B7A80)) as f32) * *(global::<f32>(0x017ACCE8));
        let lim1 = if 0.0 > raw1 {
            0.0
        } else if raw1 > hi {
            hi
        } else {
            raw1
        };
        // All four bounds must strictly contain the limits, else nothing.
        if !(lim1 > r_max_a) {
            return 0;
        }
        if !(r_min_b > lim1) {
            return 0;
        }
        if !(lim0 > r_max_c) {
            return 0;
        }
        if !(r_min_d > lim0) {
            return 0;
        }
        let _: u32 = callee_thiscall!(6, u32, event_sink, relocated(0x00F01BB8));
        let tmp2: u32 = callee_thiscall!(7, u32, ui_service, relocated(0x00F01BD8), arg0);
        let _: u32 = callee_thiscall!(8, u32, tmp2);
        let vt_this = *(this as *const u32);
        let slot = *((vt_this.wrapping_add(NOTIFY_SLOT)) as *const u32);
        let notify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _ = notify(this);
        0
    }
});
