fn main() {
    if rustversion::cfg!(before(1.81)) {
        println!("cargo:rustc-cfg=cfg_old_rust");
    }
}
