fn main() {
    // The Windows crate manually injects various functions needed to implement BSTR.
    // This test validates these are included.
    windows::build! {
        Windows::Win32::System::OleAutomation::BSTR,
    };
}
