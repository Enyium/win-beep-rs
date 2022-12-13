use windows_sys::Win32::System::Diagnostics::Debug::Beep;

pub fn beep(hertz: u32, millis: u32) {
    //! Beeps synchronously.

    unsafe { Beep(hertz, millis) };
}

#[cfg(test)]
mod tests {
    use windows_sys::{Win32::{UI::WindowsAndMessaging::{MessageBoxA, IDYES, MB_YESNO}}};

    use crate::beep;

    #[test]
    fn confirmed_beep() {
        beep(1000, 100);

        // 🤓
        let answer = unsafe { MessageBoxA(0, "Did you hear a beep?\n\n(System sounds must not be muted.)\0".as_ptr(), "Test\0".as_ptr(), MB_YESNO) };
        assert_eq!(answer, IDYES);
    }
}
