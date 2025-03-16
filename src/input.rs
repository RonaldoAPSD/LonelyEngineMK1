
#[cfg(windows)]
mod windows_input {
    use std::io;
    use std::collections::HashSet;
    use winapi::um::consoleapi::{GetNumberOfConsoleInputEvents, ReadConsoleInputW};
    use winapi::um::wincon::{INPUT_RECORD, KEY_EVENT_RECORD};

    /// Represents a key press.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Key {
        Char(char),
        Up,
        Down,
        Left,
        Right,
        Space,
        Enter,
        Shift,
        Ctrl,
        Esc,
        Unknown,
    }

    /// Reads all currently pressed keys
    pub fn read_active_keys() -> io::Result<HashSet<Key>> {
        let mut keys = HashSet::new();
        unsafe {
            let handle = winapi::um::processenv::GetStdHandle(winapi::um::winbase::STD_INPUT_HANDLE);
            let mut num_events = 0;

            if GetNumberOfConsoleInputEvents(handle, &mut num_events) == 0 {
                return Err(io::Error::last_os_error());
            }

            for _ in 0..num_events {
                let mut input_record: INPUT_RECORD = std::mem::zeroed();
                let mut events_read = 0;

                if ReadConsoleInputW(handle, &mut input_record, 1, &mut events_read) != 0 {
                    if input_record.EventType == winapi::um::wincon::KEY_EVENT {
                        let key_event = *input_record.Event.KeyEvent();
                        if key_event.bKeyDown != 0 {
                            match key_code_to_key(&key_event) {
                                Ok(key) => { keys.insert(key); },
                                Err(_) => { continue; },
                            }
                        }
                    }
                }
            }
        }

        Ok(keys)
    }

    /// Reads a key press from stdin.
    pub fn read_key() -> io::Result<Key> {
        let keys = read_active_keys()?;
        keys.into_iter().next().ok_or(io::Error::new(io::ErrorKind::WouldBlock, "No input available"))
    }

    fn key_code_to_key(key_event: &KEY_EVENT_RECORD) -> io::Result<Key> {
        let virtual_key_code = key_event.wVirtualKeyCode;
        Ok(match virtual_key_code {
            x if x == winapi::um::winuser::VK_UP as u16 => Key::Up,
            x if x == winapi::um::winuser::VK_DOWN as u16 => Key::Down,
            x if x == winapi::um::winuser::VK_LEFT as u16 => Key::Left,
            x if x == winapi::um::winuser::VK_RIGHT as u16 => Key::Right,
            x if x == winapi::um::winuser::VK_SPACE as u16 => Key::Space,
            x if x == winapi::um::winuser::VK_RETURN as u16 => Key::Enter,
            x if x == winapi::um::winuser::VK_SHIFT as u16 => Key::Shift,
            x if x == winapi::um::winuser::VK_CONTROL as u16 => Key::Ctrl,
            x if x == winapi::um::winuser::VK_ESCAPE as u16 => Key::Esc,
            _ => {
                unsafe {
                    if *key_event.uChar.UnicodeChar() != 0 {
                        Key::Char(*key_event.uChar.UnicodeChar() as u8 as char)
                    } else {
                        Key::Unknown
                    }
                }
            }
        })
    }
}

#[cfg(not(windows))]
mod unix_input {
    use std::io;

    /// Represents a key press.
    pub enum Key {
        Char(char),
        Up,
        Down,
        Left,
        Right,
        Esc,
        Unknown,
    }

    /// Give error as read_key is not implemented for other platforms
    pub fn read_key() -> io::Result<Key> {
        Err(io::Error::new(io::ErrorKind::Other, "Input not implemented for non-Windows platforms"))
    }
}

#[cfg(windows)]
pub use windows_input::*;

#[cfg(not(windows))]
pub use unix_input::*;