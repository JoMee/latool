use std::fs::File;
use std::io::Write;

pub fn generate_homework() {
    let style_file_content = include_str!("templates/homework/homework.cls");
    let main_file_content = include_str!("templates/homework/main.tex");
    let make_file_content = include_str!("templates/homework/Makefile");

    let mut style_file = File::create("homework.cls").expect("Failed to create output file");
    let mut main_file = File::create("main.tex").expect("Failed to create output file");
    let mut make_file = File::create("Makefile").expect("Failed to create output file");

    style_file.write_all(style_file_content.as_bytes()).expect("Failed to write to output file");
    main_file.write_all(main_file_content.as_bytes()).expect("Failed to write to output file");
    make_file.write_all(make_file_content.as_bytes()).expect("Failed to write to output file");

    println!("Homework sheet generated successfully!");
}

