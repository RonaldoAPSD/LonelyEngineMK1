use std::io;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

#[cfg(windows)]
mod windows_audio {
    use super::*;
    use windows::Win32::Media::Audio::{PlaySoundW, SND_FILENAME, SND_ASYNC};
    use windows::Win32::Foundation::PWSTR;
    

    /// Plays a WAV file asynchronosly on Windows using the PlaySoundW API.
    pub fn play_sound(file: &str) -> io::Result<()> {
        // Convert the file path to a wide (UTF-16) string required by PlaySoundW.
        let wide: Vec<u16> = OsStr::new(file)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Call PlaySoundW with the SND_FILENAME and SND_ASYNC flags.
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

    /// Stub implementation for non-Windows platforms.
    pub fn play_sound(_file: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "Audio not implement for non-Window platforms"))
    }
}

#[cfg(windows)]
pub use windows_audio::*;

#[cfg(not(windows))]
pub use unix_audio::*;