fn main() {
	println!("cargo:rustc-link-search=native=..");
	println!("cargo:rustc-link-lib=asm");
	println!("cargo:rerun-if-changed=../libasm.a");
}
