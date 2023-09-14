fn main() {
    tonic_build::compile_protos("proto/TelemetryWriter.proto").unwrap();
    tonic_build::compile_protos("proto/TelemetryReader.proto").unwrap();
}
