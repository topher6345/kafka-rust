# Hello Kafka in Rust

This project is a proof-of-concept of a Rust application reading from a Kafka topic, it will consume messages from a topic and print the messages to standard out

## Getting Started

* Start a kafka server on your local machine by following the Kafka Quickstart guider - https://kafka.apache.org/quickstart

Version Kafka 2.11-1.1.0

* once the kafka server is working, and can produce and consume messages:
* make sure you have Rust `1.23.0` installed by running `rustc 1.23.0` see https://www.rust-lang.org/en-US/install.html
* pull down this repo
* in this repo run `cargo build`
* if you have followed the directions and started the kafka brokers on `localhost:9092` and created the topic `test` then you can start the rust application with `cargo run  -- --brokers localhost:9092 --topics test`
* broadcast the messages in a seperate terminal window using the `bin/kafka-console-producer.sh --broker-list localhost:9092 --topic test` script 
* assert that you see messages echo'd to the standard out in the Rust applicaiton


