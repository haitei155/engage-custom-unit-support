//! The companion UI bundles provide 22 bond entries and 30 navigation slots.
//! Keep entry setup native; protect only the missing-indicator error path.

pub fn install() {
    // SetNavi's bounds check branches here only when the indicator is absent.
    // Return through its existing epilogue instead of calling ThrowHelper.
    const ERROR_PATH: usize = 0x24a512c;
    const EPILOGUE: usize = 0x24a5320;
    let guards: &[(usize, u32)] = &[
        (0x24a5124, 0x6b15011f),
        (0x24a5128, 0x54000068),
        (ERROR_PATH, 0xaa1f03e0),
        (EPILOGUE, 0x6d4123e9),
    ];
    let base = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize };
    if !guards.iter().all(|&(off, word)| unsafe {
        std::ptr::read_unaligned((base + off) as *const u32) == word
    }) { return; }
    let branch = 0x14000000u32 | (((EPILOGUE - ERROR_PATH) / 4) as u32);
    let _ = skyline::patching::Patch::in_text(ERROR_PATH).data(branch);
}
