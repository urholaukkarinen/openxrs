fn main() {
    #[cfg(feature = "static")]
    {
        let dst = cmake::Config::new("OpenXR-SDK")
            .define("BUILD_API_LAYERS", "OFF")
            .define("BUILD_TESTS", "OFF")
            .define("OpenGL_GL_PREFERENCE", "GLVND")
            .define("DYNAMIC_LOADER", "OFF")
            .define("CMAKE_INSTALL_LIBDIR", "lib")
            .define("_DISABLE_CONSTEXPR_MUTEX_CONSTRUCTOR", "0")
            .profile("Release")
            .build();

        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib=static=openxr_loader");

        let target_os = std::env::var_os("CARGO_CFG_TARGET_OS")
            .expect("missing CARGO_CFG_TARGET_OS environment variable");

        if target_os == "macos" || target_os == "freebsd" {
            println!("cargo:rustc-link-lib=c++");
        } else if target_os != "windows" {
            println!("cargo:rustc-link-lib=stdc++");
        }
    }
    #[cfg(all(not(feature = "static"), feature = "linked"))]
    {
        println!("cargo:rustc-link-lib=openxr_loader");
    }
}
