#![cfg(target_os = "windows")]
#![warn(clippy::pedantic)]

use windows_sys::Win32::System::Diagnostics::Debug::Beep;

pub fn beep_with_hz_and_millis(hertz: u32, millis: u32) {
    //! Beeps synchronously. System sounds must not be muted for the beep to be audible.

    unsafe { Beep(hertz, millis) };
}

#[cfg(test)]
mod tests {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxA, IDYES, MB_YESNO};

    use crate::beep_with_hz_and_millis;

    #[test]
    fn confirmed_beep() {
        beep_with_hz_and_millis(1000, 100);

        // 🤓
        let answer = unsafe {
            MessageBoxA(
                0,
                "Did you hear a beep?\n\n(System sounds must not be muted.)\0".as_ptr(),
                "Test\0".as_ptr(),
                MB_YESNO,
            )
        };
        assert_eq!(answer, IDYES);
    }
}
