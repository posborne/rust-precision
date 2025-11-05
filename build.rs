#[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
fn asm_detect() {
    let asm_capable_target = cfg!(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64",
    ));
    if asm_capable_target {
        println!("cargo:rustc-cfg=asm");
    } else {
        cc::Build::new()
            .file("src/cpucounter.c")
            .flag_if_supported("-fomit-frame-pointer")
            .opt_level(3)
            .compile("cpucounter");
    }
}

fn main() {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    {
        let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH");
        match target_arch {
            Err(_) => {}
            Ok(ref arch) if arch == "wasm32" || arch == "wasm64" => {}
            Ok(_) => asm_detect(),
        }
    }
}
