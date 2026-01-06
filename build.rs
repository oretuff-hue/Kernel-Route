fn main() {
    cc::Build::new()
        .file("src/multiboot.S")
        .compile("multiboot");
}        
