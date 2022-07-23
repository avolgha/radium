use cc;

fn main() {
    cc::Build::new()
        .file("src/read_line.c")
        .compile("read_line_lib");
}
