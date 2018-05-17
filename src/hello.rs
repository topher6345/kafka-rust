pub mod hello {
  use kafka::producer::{Producer, Record, RequiredAcks};
  use std::time::Duration;
  use kafka::error::Error as KafkaError;
  use json;
  use std::str;

  pub fn produce_message<'a, 'b>(
      data: &'a [u8],
      topic: &'b str,
      brokers: Vec<String>,
  ) -> Result<(), KafkaError> {
      println!("About to publish a message at {:?} to: {}", brokers, topic);

      // ~ create a producer. this is a relatively costly operation, so
      // you'll do this typically once in your application and re-use
      // the instance many times.
      let mut producer =
          Producer::from_hosts(brokers)
               // ~ give the brokers one second time to ack the message
               .with_ack_timeout(Duration::from_secs(1))
               // ~ require only one broker to ack the message
               .with_required_acks(RequiredAcks::One)
               // ~ build the producer with the above settings
               .create()?;

      // ~ now send a single message.  this is a synchronous/blocking
      // operation.

      // ~ we're sending 'data' as a 'value'. there will be no key
      // associated with the sent message.

      // ~ we leave the partition "unspecified" - this is a negative
      // partition - which causes the producer to find out one on its
      // own using its underlying partitioner.
      producer.send(&Record {
          topic: topic,
          partition: -1,
          key: (),
          value: data,
      })?;

      // ~ we can achieve exactly the same as above in a shorter way with
      // the following call
      producer.send(&Record::from_value(topic, data))?;

      Ok(())
  }

  pub fn process_value(value: &[u8]) -> () {
    let string = str::from_utf8(value).unwrap();
    if json::parse(string).unwrap()["foo"].is_object() {
        let broker = "localhost:9092";
        let topic = "greetings";
        if let Err(e) = produce_message(value, topic, vec![broker.to_owned()]) {
            println!("Failed producing messages: {}", e);
        }
    }
}

}