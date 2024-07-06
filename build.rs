fn main() {
	println!("cargo:rustc-link-search=native=..");
	println!("cargo:rerun-if-changed=libasm.a");
	println!("cargo:rerun-if-changed=libasm_bonus.a");
}
