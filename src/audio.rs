//! Platform-specific audio playback implementation.
//!
//! Provides functionality for playing sound effects using native system APIs.
//! Currently supports WAV file playback on Windows via the Win32 API.
//! Non-Windows platforms have a stub implementation that returns errors.

use std::io;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
mod windows_audio {
    use super::*;
    use windows::Win32::Media::Audio::{PlaySoundW, SND_FILENAME, SND_ASYNC};
    use windows::Win32::Foundation::PWSTR;
    

    /// Plays a WAV file asynchronously using the Windows PlaySoundW API.
    ///
    /// # Arguments
    /// * `file` - Path to the WAV file to play. Must be valid UTF-8.
    ///
    /// # Returns
    /// * `Ok(())` if sound playback started successfully
    /// * `Err(io::Error)` if playback failed
    ///
    /// # Safety
    /// This function contains unsafe code for Win32 API calls.
    ///
    /// # Platform Specific
    /// Windows only. Requires valid WAV file path.
    ///
    /// # Example
    /// ```no_run
    /// use lonely_engine::audio;
    ///
    /// if let Err(e) = audio::play_sound("sound.wav") {
    ///     eprintln!("Error playing sound: {}", e);
    /// }
    /// ```
    pub fn play_sound(file: &str) -> io::Result<()> {
        // Convert the file path to a wide (UTF-16) string required by PlaySoundW.
        let wide: Vec<u16> = OsStr::new(file)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // SAFETY: We ensure the wide string is properly null-terminated and
        // valid for the duration of the PlaySoundW call
        let result = unsafe {
            PlaySoundW(PWSTR(wide.as_ptr() as *mut u16), None, SND_FILENAME as u32 | SND_ASYNC as u32)
        };

        // If the result if 0, the function failed.
        if !result.as_bool() {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to play sound"))
        } else {
            Ok(())
        }
    }
}

#[cfg(not(windows))]
mod unix_audio {
    use std::io;

    /// Stub implementation for non-Windows platforms
    ///
    /// # Platform Specific
    /// Always returns an error on non-Windows platforms
    ///
    /// # Note
    /// This is a placeholder implementation. Consider using platform-specific
    /// audio libraries (e.g., ALSA, PulseAudio) for Unix support.
    pub fn play_sound(_file: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Audio not implement for non-Window platforms"))
    }
}

#[cfg(windows)]
pub use windows_audio::*;

#[cfg(not(windows))]
pub use unix_audio::*;