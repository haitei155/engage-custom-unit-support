//! Minimal standalone missing-pair-label fallback. No outfit or sequence-stage hooks.
#[cfg(feature = "diagnostics")]
use std::io::Write;
type P = *mut u8;
macro_rules! trace {
    ($message:expr) => {{
        #[cfg(feature = "diagnostics")]
        log($message);
    }};
}

#[cfg(feature = "diagnostics")]
fn log(s: &str) {
    let line=format!("[FEE Dining v3] {s}\n");
    let _=horizon_svc::output_debug_string(&line);
    if let Ok(mut f)=std::fs::OpenOptions::new().create(true).append(true).open("sd:/engage/dining-pair-trace-v3.log") { let _=f.write_all(line.as_bytes()); }
}
unsafe fn read<T: Copy>(p: P, offset: usize) -> T { std::ptr::read_unaligned(p.add(offset) as *const T) }
#[cfg(feature = "diagnostics")]
unsafe fn string(p: P) -> String {
    if p.is_null() { return "<null>".into(); }
    let n = read::<i32>(p, 0x10);
    if !(0..=256).contains(&n) { return "<invalid-string-length>".into(); }
    String::from_utf16_lossy(std::slice::from_raw_parts(p.add(0x14) as *const u16, n as usize))
}
#[skyline::hook(offset = 0x2d830e0)]
unsafe fn fee_dining_v3_label(p:P,unit:P,order:i32,other:P,m:P)->P {
    trace!(&format!("GetConversationLabel enter order={order} type={}",read::<i32>(p,0xb0)));
    let mut r=call_original!(p,unit,order,other,m);
    if read::<i32>(p,0xb0)==2 && (r.is_null() || read::<i32>(r,0x10)==0) {
        // Same helper used by the native type-0 branch. It derives A/B/C from
        // the actual dish and this unit's tastes; no fabricated pair/name index.
        let taste: unsafe extern "C" fn(P,P,P)->P = std::mem::transmute(
            skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize+0x2d83220);
        r=taste(p,unit,std::ptr::null_mut());
        trace!(&format!("PAIR_LABEL_MISSING order={order}; native taste fallback={}",string(r)));
    }
    trace!(&format!("GetConversationLabel exit label={}",string(r)));r
}
#[skyline::main]
pub fn main() {
    let guards: &[(usize,u32)] = &[
        (0x2d830e0,0xa9bc7bfd),
        (0x2d830e4,0xf9000bf7),
        (0x2d830e8,0x910003fd),
        (0x2d830ec,0xa90257f6),
        (0x2d83220,0xa9bc7bfd),
        (0x2d83224,0xa9015ff8),
        (0x2d83228,0x910003fd),
        (0x2d8322c,0xa90257f6),
    ];
    let base=unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize };
    if !guards.iter().all(|&(off,word)|unsafe {std::ptr::read_unaligned((base+off) as *const u32)==word}) {
        trace!("unsupported executable: no hook installed");return;
    }
    skyline::install_hooks!(fee_dining_v3_label);
    trace!("installed ONE label fallback hook; no cooking-stage or outfit hooks");
}
