FROM debian:stable-slim
WORKDIR /app

ARG execfile=target/release
ADD $execfile/Mqtt2InfluxDb Mqtt2InfluxDb
RUN chmod +x Mqtt2InfluxDb
CMD ["./Mqtt2InfluxDb"]
