# code-generator
This is a simple project for generating QR codes from text.

## Description
This project uses the Actix web framework and the `qrcode_generator` crate to create a web server that generates QR codes. It exposes a POST endpoint at "/qr" that accepts JSON data with a "text" field. The server generates a QR code from this text and returns the QR code as a PNG image.

## How to use
First, clone the repository and navigate into the project directory. Then, run the server using Cargo:

```bash
cargo run
```
Once the server is running, you can generate a QR code by sending a POST request to "http://localhost:8080/qr" with JSON data containing the text. Here's an example using curl:

```bash
curl -X POST -H "Content-Type: application/json" -d '{"text":"Hello, world!"}' http://localhost:8080/qr > qrcode.png
```

This command sends a POST request with the text "Hello, world!" and saves the returned QR code to a file named "qrcode.png".

## Dependencies

* [Actix Web](htthttps://actix.rs/)
* [qrcode_generator](https://crates.io/crates/qrcode_generator)

## License

[MIT](LICENSE)

