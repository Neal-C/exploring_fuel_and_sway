# I take no responsibility for bloated Docker images. Proceed at your own risk :)
FROM rust AS fuel

RUN apt-get update && apt-get install -y curl gzip tar

RUN curl -L -o fuelup.tar.gz https://github.com/FuelLabs/fuelup/releases/download/v0.27.3/fuelup-0.27.3-x86_64-unknown-linux-gnu.tar.gz

RUN tar -xvf fuelup.tar.gz

RUN mv fuelup-0.27.3-x86_64-unknown-linux-gnu/fuelup /usr/local/bin/fuelup

RUN chmod +x usr/local/bin/fuelup

ENV PATH=$PATH:/usr/local/bin/fuelup

# this will likely break the image as soon as latest is no longer forc 0.68.9
RUN fuelup toolchain install latest

ENV PATH=$PATH:/root/.fuelup/bin

FROM fuel 

WORKDIR /app

COPY . .

CMD ["forc", "build"]

