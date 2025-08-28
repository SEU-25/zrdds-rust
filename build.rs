use glob::glob;
use std::fs;
use std::path::PathBuf;

fn main() {
    let include_dir = PathBuf::from(r"include");

    println!("include dir: {}", include_dir.display());
    println!("Searching for header files...");

    let mut builder = bindgen::Builder::default()
        // 添加两个 include 路径
        .clang_arg(format!("-I{}/CInterface", include_dir.display()))
        .clang_arg(format!("-I{}/ZRDDSCoreInterface", include_dir.display()))
        .generate_comments(false);

    // 遍历 CInterface 下所有 .h
    for path in glob(&format!("{}/CInterface/*.h", include_dir.display()))
        .expect("Failed to read glob pattern").flatten()
    {
        builder = builder.header(path.to_string_lossy());
    }

    // 遍历 ZRDDSCoreInterface 下所有 .h
    for path in glob(&format!("{}/ZRDDSCoreInterface/*.h", include_dir.display()))
        .expect("Failed to read glob pattern").flatten()
    {
        builder = builder.header(path.to_string_lossy());
    }

    // 生成绑定
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_file = "src/bindings.rs";
    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");

    // 读取文件内容
    let content = fs::read_to_string(out_file).expect("Failed to read bindings.rs");

    // 在文件开头加上 #![allow(warnings)]
    let mut new_content = String::from("#![allow(warnings)]\n");

    // 替换 extern "C" 部分
    let replaced_content = content.replace(
        "unsafe extern \"C\" {",
        "#[link(name = \"ZRDDSC_VS2019\")]\nunsafe extern \"C\" {",
    );

    new_content.push_str(&replaced_content);

    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-lib=dylib=ZRDDSC_VS2019");

    fs::write(out_file, new_content).expect("Failed to write modified bindings.rs");
}
