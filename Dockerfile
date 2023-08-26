FROM ubuntu:22.04
COPY ./target/release/my-telemetry-server ./target/release/my-telemetry-server
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/my-telemetry-server"]