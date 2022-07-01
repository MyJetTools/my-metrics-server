FROM rust:slim
COPY ./target/release/services-status ./target/release/services-status 
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/services-status"]