fn main() {
    if cfg!(not(target_os = "linux")) {
        panic!("This crate can be compiled only for Linux");
    }
}
