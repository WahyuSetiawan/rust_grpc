<<<<<<< HEAD
fn main()->Result<(), Box<dyn std::error::Error>>{
    tonic_build::compile_protos("proto/say.proto").unwrap();
    Ok(())
}
=======
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    tonic_build::compile_protos("proto/user.proto")?;
    return Ok(());
}
>>>>>>> 7f6a2292e9bdf48fda7c93093588515a7ef25fc4
