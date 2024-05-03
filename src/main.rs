mod models;
mod json_examples;
mod custom_serializer;


fn main() {
    // JSON examples
    println!("Running JSON serialization example...");
    let serialized = json_examples::serialize_user();
    println!("Serialized User: {}", serialized);

    println!("Running JSON deserialization example...");
    let user = json_examples::deserialize_user(&serialized);
    println!("Deserialized User: {:?}", user);


}
