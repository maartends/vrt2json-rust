# vrt2json-rust

## Synopsis

Replacement-to-be for [https://github.com/viaacode/vrt2json](https://github.com/viaacode/vrt2json).

We need:

- An AMQP client library: [https://github.com/sozu-proxy/lapin](https://github.com/sozu-proxy/lapin)
- An xml2JSON library: [https://github.com/sacooper/xmlJSON-rs](https://github.com/sacooper/xmlJSON-rs)
- A data-transformation library, but this could be plain XSLT whereafter the proper XML-output can be transformed into JSON with the aformentioned library.

Ideally, also:
- Some structured logging: [https://github.com/slog-rs](https://github.com/slog-rs) seems to be the way to go.

Some helpful links:
- AMQP: [https://www.clever-cloud.com/blog/engineering/2017/03/28/lapin-new-rust-amqp-library/](https://www.clever-cloud.com/blog/engineering/2017/03/28/lapin-new-rust-amqp-library/)
- Structured logging: [http://siciarz.net/24-days-rust-structured-logging/](http://siciarz.net/24-days-rust-structured-logging/)
