FROM ubuntu:22.04
COPY ./target/release/my-telemetry-server ./my-telemetry-server
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./my-telemetry-server"]