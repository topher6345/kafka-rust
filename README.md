# Hello Kafka in Rust

This project is a proof-of-concept cargo project for a rust project reading from a Kafka topic, it will consume messages from a topic as JSON and conditionally emit messages on another topic

The rust code is taken from https://github.com/spicavigo/kafka-rust/blob/master/examples/console-consumer.rs

## Getting Started

* Start a kafka server on your local machine by following the Kafka Quickstart guide - https://kafka.apache.org/quickstart

Version Kafka 2.11-1.1.0

* once the kafka server is working, and can produce and consume messages:
* make sure you have Rust `1.23.0` installed by running `rustc 1.23.0` see https://www.rust-lang.org/en-US/install.html
* pull down this repo
* in this repo run `cargo build`
* `bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic greetings`
* `bin/kafka-topics.sh --create --zookeeper localhost:2181 --replication-factor 1 --partitions 1 --topic test`
* if you have followed the directions and started the kafka brokers on `localhost:9092` and created the topic `test` then you can start the rust application with `cargo run  -- --brokers localhost:9092 --topics test`
* broadcast the messages in a seperate terminal window using the `bin/kafka-console-producer.sh --broker-list localhost:9092 --topic test` script 
* start a consumer for the `greetings` topic `bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic greetings --from-beginning`

* assert that you see messages echo'd by the greetings topic consumer


