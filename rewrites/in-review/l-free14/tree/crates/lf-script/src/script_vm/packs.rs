//! Two pack-and-forward singles: the loading clock and the angled area.
//!
//! These two verified routines share no state with anything else in the
//! group, so they share this module (one or two routines is not a
//! structure of its own). [`LoadingClock`] packs nine float words and one
//! integer into a ten-word frame and presents five sliding windows into
//! it with a packed colour; [`AngledArea`] packs two triples and forwards
//! them in order with a trailing zero.

// Signatures mirror the 32-bit routines' words one by one, so long
// argument lists are inherent here.
#![allow(clippy::too_many_arguments)]

/// Draws the loading clock: the clock routine's callee.
///
/// The five windows are views into one ten-word frame: 2, 4, 6, 8 and 10
/// words, each running to the frame end.
pub trait ClockDraw {
    /// Draws the windows with the integer, the packed colour and the two
    /// trailing integers.
    fn draw(
        &mut self,
        win2: [u32; 2],
        win4: [u32; 4],
        win6: [u32; 6],
        win8: [u32; 8],
        win10: [u32; 10],
        int: u32,
        color: u32,
        tail0: u32,
        tail1: u32,
    );
}

/// Clears an angled area: the angled routine's callee.
pub trait AngledClear {
    /// Clears between the two triples with `flag` and a trailing zero.
    fn clear(&mut self, near: [u32; 3], far: [u32; 3], flag: u32, zero: u32);
}

impl<F: FnMut([u32; 2], [u32; 4], [u32; 6], [u32; 8], [u32; 10], u32, u32, u32, u32)> ClockDraw
    for F
{
    fn draw(
        &mut self,
        win2: [u32; 2],
        win4: [u32; 4],
        win6: [u32; 6],
        win8: [u32; 8],
        win10: [u32; 10],
        int: u32,
        color: u32,
        tail0: u32,
        tail1: u32,
    ) {
        self(win2, win4, win6, win8, win10, int, color, tail0, tail1);
    }
}

impl<F: FnMut([u32; 3], [u32; 3], u32, u32)> AngledClear for F {
    fn clear(&mut self, near: [u32; 3], far: [u32; 3], flag: u32, zero: u32) {
        self(near, far, flag, zero);
    }
}

/// The loading-clock pack: nine float words, one integer, a colour.
#[derive(Debug, Default, Clone, Copy)]
pub struct LoadingClock;

impl LoadingClock {
    /// Packs the frame and draws the five windows.
    ///
    /// The frame holds (`f8`, `f9`, `f6`, `f7`, `f4`, `a5`, `f2`, `f3`,
    /// `f0`, `f1`) in order; the windows start at frame words 8, 6, 4, 2
    /// and 0. The colour packs the low bytes of (`c14`, `c11`, `c12`,
    /// `c13`) from the top byte down.
    pub fn draw(
        &self,
        sink: &mut impl ClockDraw,
        f0: u32,
        f1: u32,
        f2: u32,
        f3: u32,
        f4: u32,
        a5: u32,
        f6: u32,
        f7: u32,
        f8: u32,
        f9: u32,
        i10: u32,
        c11: u32,
        c12: u32,
        c13: u32,
        c14: u32,
        i15: u32,
        i16: u32,
    ) {
        let frame = [f8, f9, f6, f7, f4, a5, f2, f3, f0, f1];
        let color =
            ((c14 & 0xFF) << 24) | ((c11 & 0xFF) << 16) | ((c12 & 0xFF) << 8) | (c13 & 0xFF);
        sink.draw(
            [frame[8], frame[9]],
            [frame[6], frame[7], frame[8], frame[9]],
            [frame[4], frame[5], frame[6], frame[7], frame[8], frame[9]],
            [
                frame[2], frame[3], frame[4], frame[5], frame[6], frame[7], frame[8], frame[9],
            ],
            frame,
            i10,
            color,
            i15,
            i16,
        );
    }
}

/// The angled-area pack: two triples forwarded in order.
#[derive(Debug, Default, Clone, Copy)]
pub struct AngledArea;

impl AngledArea {
    /// Packs (`a0`, `a1`, `a2`) and (`a3`, `a4`, `a5`) and clears between
    /// them with `flag` and a trailing zero.
    pub fn clear(
        &self,
        sink: &mut impl AngledClear,
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        flag: u32,
    ) {
        sink.clear([a0, a1, a2], [a3, a4, a5], flag, 0);
    }
}
