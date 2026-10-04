//! Audio output: device enumeration, sample-rate and channel negotiation,
//! and pull-model streams.
//!
//! # Model
//!
//! An [`AudioOutput`] lists devices ([`DeviceInfo`]) and opens streams on
//! them. A stream is pull-model: the backend owns the timing and calls the
//! stream's [`AudioRender`] whenever the device needs more sound. The engine
//! mixer is that renderer; nothing pushes samples.
//!
//! # The render contract
//!
//! - Each call receives `out`, a buffer of `frames * channels` samples,
//!   interleaved frame by frame (for stereo: left, right, left, right, ...),
//!   using the stream's negotiated [`StreamConfig`].
//! - `out` is zero-filled before every call, so a renderer that writes
//!   nothing produces silence.
//! - `frames` is between 1 and the negotiated `buffer_frames`; it can vary
//!   from call to call. [`RenderInfo::frame_position`] counts the frames
//!   rendered before this call, so a renderer can keep sample-exact time.
//! - Samples are nominally in `-1.0..=1.0`. Backends that convert to integer
//!   formats clamp; the capture device records values exactly as written.
//! - The renderer runs on the backend's audio thread (OS backends) or in the
//!   caller's thread ([`CaptureAudio::pump`]). It must not block, take locks
//!   that the game thread holds for long, or allocate: an audio thread that
//!   misses its deadline is heard as a click.
//!
//! # Negotiation
//!
//! The engine asks for a [`StreamConfig`]; [`negotiate`] turns it into one
//! the device supports, and that is what the stream runs at. The mixer
//! resamples and up- or down-mixes to the negotiated format. The rules,
//! applied by every backend through [`negotiate`]:
//!
//! - Sample rate: the requested rate if the device has it; otherwise the
//!   lowest supported rate above it (upsampling keeps quality); otherwise
//!   the highest supported rate.
//! - Channels: the requested count if supported; otherwise the lowest count
//!   above it (the mixer up-mixes, putting stereo on the front pair);
//!   otherwise the highest (the mixer down-mixes).
//! - Buffer: the requested frame count clamped to the device's range; zero
//!   asks for the device's default.
//!
//! # Where an OS backend plugs in
//!
//! There is no OS backend yet. The plan is SDL3 (it also provides windowing
//! and gamepads), which needs a download and waits for approval. When it is
//! approved, `audio/sdl3.rs` in this crate implements [`AudioOutput`]:
//! `devices` maps SDL's playback device list to [`DeviceInfo`],
//! `open_stream` calls [`negotiate`], opens an SDL audio stream in 32-bit
//! float format at the negotiated rate and channel count, and installs a
//! callback that zero-fills SDL's request buffer, calls the [`AudioRender`]
//! and advances the frame position. Device removal maps to
//! [`StreamState::Lost`]. Engine code does not change: it only ever sees
//! these traits. Until then, [`NullAudio`] runs the game silent and
//! [`CaptureAudio`] lets tests hear the mixer.

mod capture;
mod null;

use std::fmt;

pub use capture::CaptureAudio;
pub use null::NullAudio;

/// Identifies a device within one [`AudioOutput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AudioDeviceId(pub u32);

/// What a device supports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    /// The device's id.
    pub id: AudioDeviceId,
    /// Human-readable name for a settings menu.
    pub name: String,
    /// True for the system's default output.
    pub is_default: bool,
    /// Supported sample rates in hertz, ascending.
    pub sample_rates: Vec<u32>,
    /// Supported channel counts, ascending.
    pub channel_counts: Vec<u16>,
    /// Smallest buffer the device accepts, in frames.
    pub min_buffer_frames: u32,
    /// Largest buffer the device accepts, in frames.
    pub max_buffer_frames: u32,
    /// Buffer used when a request asks for zero frames.
    pub default_buffer_frames: u32,
}

/// A stream format: rate, channel count and buffer size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StreamConfig {
    /// Sample rate in hertz.
    pub sample_rate_hz: u32,
    /// Interleaved channels per frame.
    pub channels: u16,
    /// Frames per render call at most (0 in a request means the device's
    /// default).
    pub buffer_frames: u32,
}

impl StreamConfig {
    /// Stereo at 48 kHz with the device's default buffer: the usual request.
    pub const STEREO_48K: StreamConfig = StreamConfig {
        sample_rate_hz: 48_000,
        channels: 2,
        buffer_frames: 0,
    };
}

/// Facts passed to every render call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderInfo {
    /// The stream's negotiated format.
    pub config: StreamConfig,
    /// Frames rendered by this stream before this call.
    pub frame_position: u64,
    /// Frames in this call (`out.len() / channels`).
    pub frames: usize,
}

/// The pull-model renderer: fills interleaved `f32` frames on demand. See
/// the module docs for the contract.
pub trait AudioRender: Send {
    /// Fills `out` (`info.frames * channels` samples, zeroed on entry).
    fn render(&mut self, out: &mut [f32], info: &RenderInfo);
}

impl<F: FnMut(&mut [f32], &RenderInfo) + Send> AudioRender for F {
    fn render(&mut self, out: &mut [f32], info: &RenderInfo) {
        self(out, info);
    }
}

/// Whether a stream is producing sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamState {
    /// Opened or paused: the renderer is not called.
    Paused,
    /// Playing: the renderer is called on the backend's schedule.
    Playing,
    /// The device went away; the stream cannot play again. Open a new one.
    Lost,
}

/// Audio errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    /// No device to open (none listed, or no default).
    NoDevice,
    /// The id names no device of this output.
    UnknownDevice(AudioDeviceId),
    /// The request or the device cannot be satisfied.
    Unsupported(&'static str),
    /// The stream's device went away.
    StreamLost,
    /// The OS backend reported an error.
    Backend(String),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::NoDevice => f.write_str("no audio output device"),
            AudioError::UnknownDevice(id) => write!(f, "unknown audio device {}", id.0),
            AudioError::Unsupported(why) => write!(f, "unsupported audio format: {why}"),
            AudioError::StreamLost => f.write_str("the audio device was lost"),
            AudioError::Backend(msg) => write!(f, "audio backend error: {msg}"),
        }
    }
}

impl std::error::Error for AudioError {}

/// Picks the stream format a device will run, by the rules in the module
/// docs.
///
/// # Errors
///
/// Returns [`AudioError::Unsupported`] for a request with a zero rate or
/// zero channels, or a device that lists no rates or no channel counts.
pub fn negotiate(device: &DeviceInfo, wanted: StreamConfig) -> Result<StreamConfig, AudioError> {
    if wanted.sample_rate_hz == 0 || wanted.channels == 0 {
        return Err(AudioError::Unsupported(
            "zero sample rate or channel count requested",
        ));
    }
    let sample_rate_hz = pick(&device.sample_rates, wanted.sample_rate_hz)
        .ok_or(AudioError::Unsupported("device lists no sample rates"))?;
    let channels = pick(&device.channel_counts, wanted.channels)
        .ok_or(AudioError::Unsupported("device lists no channel counts"))?;
    let lo = device.min_buffer_frames.max(1);
    let hi = device.max_buffer_frames.max(lo);
    let asked = if wanted.buffer_frames == 0 {
        device.default_buffer_frames
    } else {
        wanted.buffer_frames
    };
    Ok(StreamConfig {
        sample_rate_hz,
        channels,
        buffer_frames: asked.clamp(lo, hi),
    })
}

/// Exact match, else the lowest value above, else the highest value.
fn pick<T: Copy + Ord>(supported: &[T], wanted: T) -> Option<T> {
    if supported.contains(&wanted) {
        return Some(wanted);
    }
    let above = supported.iter().copied().filter(|&v| v > wanted).min();
    above.or_else(|| supported.iter().copied().max())
}

/// An audio output: lists devices and opens streams.
pub trait AudioOutput: Send + Sync {
    /// Every device that can play, in a stable order.
    fn devices(&self) -> Vec<DeviceInfo>;

    /// The system's default device, if any.
    fn default_device(&self) -> Option<AudioDeviceId> {
        let devices = self.devices();
        devices
            .iter()
            .find(|d| d.is_default)
            .or(devices.first())
            .map(|d| d.id)
    }

    /// Opens a stream on `device` (the default when `None`), negotiated from
    /// `wanted`. The stream starts [`StreamState::Paused`]; `render` is
    /// called only after [`AudioStream::play`].
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::NoDevice`], [`AudioError::UnknownDevice`], a
    /// negotiation error, or a backend error.
    fn open_stream(
        &self,
        device: Option<AudioDeviceId>,
        wanted: StreamConfig,
        render: Box<dyn AudioRender>,
    ) -> Result<Box<dyn AudioStream>, AudioError>;
}

/// An open stream. Dropping it stops playback and drops its renderer; no
/// render call starts after the drop returns.
pub trait AudioStream: Send {
    /// The device it plays on.
    fn device(&self) -> AudioDeviceId;
    /// The negotiated format.
    fn config(&self) -> StreamConfig;
    /// Current state.
    fn state(&self) -> StreamState;
    /// Frames rendered so far.
    fn frames_rendered(&self) -> u64;
    /// Starts (or resumes) calling the renderer.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::StreamLost`] once the device has gone.
    fn play(&mut self) -> Result<(), AudioError>;
    /// Stops calling the renderer; the frame position is kept.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::StreamLost`] once the device has gone.
    fn pause(&mut self) -> Result<(), AudioError>;
}

/// Finds the device a request names (or the default) in a list.
fn find_device(
    devices: &[DeviceInfo],
    wanted: Option<AudioDeviceId>,
) -> Result<&DeviceInfo, AudioError> {
    match wanted {
        Some(id) => devices
            .iter()
            .find(|d| d.id == id)
            .ok_or(AudioError::UnknownDevice(id)),
        None => devices
            .iter()
            .find(|d| d.is_default)
            .or(devices.first())
            .ok_or(AudioError::NoDevice),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn device() -> DeviceInfo {
        DeviceInfo {
            id: AudioDeviceId(1),
            name: "test".into(),
            is_default: true,
            sample_rates: vec![22_050, 44_100, 48_000],
            channel_counts: vec![2, 6],
            min_buffer_frames: 64,
            max_buffer_frames: 4096,
            default_buffer_frames: 512,
        }
    }

    fn ask(rate: u32, channels: u16, buffer: u32) -> StreamConfig {
        StreamConfig {
            sample_rate_hz: rate,
            channels,
            buffer_frames: buffer,
        }
    }

    #[test]
    fn exact_requests_are_kept() {
        assert_eq!(
            negotiate(&device(), ask(44_100, 2, 256)).unwrap(),
            ask(44_100, 2, 256)
        );
    }

    #[test]
    fn rates_prefer_the_next_higher() {
        let d = device();
        assert_eq!(
            negotiate(&d, ask(32_000, 2, 256)).unwrap().sample_rate_hz,
            44_100
        );
        assert_eq!(
            negotiate(&d, ask(8_000, 2, 256)).unwrap().sample_rate_hz,
            22_050
        );
        assert_eq!(
            negotiate(&d, ask(96_000, 2, 256)).unwrap().sample_rate_hz,
            48_000
        );
    }

    #[test]
    fn channels_up_mix_then_down_mix() {
        let d = device();
        assert_eq!(negotiate(&d, ask(48_000, 1, 256)).unwrap().channels, 2);
        assert_eq!(negotiate(&d, ask(48_000, 4, 256)).unwrap().channels, 6);
        assert_eq!(negotiate(&d, ask(48_000, 8, 256)).unwrap().channels, 6);
    }

    #[test]
    fn buffers_clamp_and_default() {
        let d = device();
        assert_eq!(negotiate(&d, ask(48_000, 2, 0)).unwrap().buffer_frames, 512);
        assert_eq!(negotiate(&d, ask(48_000, 2, 1)).unwrap().buffer_frames, 64);
        assert_eq!(
            negotiate(&d, ask(48_000, 2, 1 << 20))
                .unwrap()
                .buffer_frames,
            4096
        );
    }

    #[test]
    fn impossible_requests_fail() {
        let d = device();
        assert!(negotiate(&d, ask(0, 2, 0)).is_err());
        assert!(negotiate(&d, ask(48_000, 0, 0)).is_err());
        let mut empty = device();
        empty.sample_rates.clear();
        assert!(negotiate(&empty, StreamConfig::STEREO_48K).is_err());
        let mut empty = device();
        empty.channel_counts.clear();
        assert!(negotiate(&empty, StreamConfig::STEREO_48K).is_err());
    }

    #[test]
    fn device_lookup() {
        let mut second = device();
        second.id = AudioDeviceId(2);
        second.is_default = false;
        let list = vec![second, device()];
        assert_eq!(find_device(&list, None).unwrap().id, AudioDeviceId(1));
        assert_eq!(
            find_device(&list, Some(AudioDeviceId(2))).unwrap().id,
            AudioDeviceId(2)
        );
        assert_eq!(
            find_device(&list, Some(AudioDeviceId(9))).unwrap_err(),
            AudioError::UnknownDevice(AudioDeviceId(9))
        );
        assert_eq!(find_device(&[], None).unwrap_err(), AudioError::NoDevice);
    }
}
