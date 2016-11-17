# Websnip
Self-hosted web service that saves web articles to clean PDFs.

## Usage
The easiest way to run Websnip is as a Docker container. A prebuilt image is available in the Docker Hub as `sagebind/websnip`.

Websnip uses the [Mercury] API to extract an article's unstyled content before generating a PDF, so you will need to get your own Mercury API key. Once you have one, you can run Websnip with that key by setting the `MERCURY_API_KEY` environment variable:

```
docker run -d -e MERCURY_API_KEY=<API_KEY> -p 80:80 sagebind/websnip
```

Once Websnip is running, it will begin accepting GET requests on port 80. The query portion of the URL is the URL of the article to convert. For example, to download the article at [`http://words.steveklabnik.com/a-new-introduction-to-rust`](http://words.steveklabnik.com/a-new-introduction-to-rust) as a PDF, you can make a GET request like this:

```
curl "http://localhost:80/?http://words.steveklabnik.com/a-new-introduction-to-rust" > a-new-introduction-to-rust.pdf
```

## Compiling
To compile Websnip yourself, you need Rust and Cargo installed, OpenSSL development libraries, and [wkhtmltopdf] libraries and headers installed. Then compile with Cargo:

```
cargo build
```

## License
This project is licensed under the MIT license. See the [license file](LICENSE.md) for details.


[Mercury]: https://mercury.postlight.com/web-parser/
[wkhtmltopdf]: http://wkhtmltopdf.org/index.html
