fn main() {
    use qrcode_generator::QrCodeEcc;

    qrcode_generator::to_png_to_file("Hello world!", QrCodeEcc::Low, 1024, "tmp/file_output.png")
        .unwrap();
}
