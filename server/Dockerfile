FROM rust AS base

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=dev

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code/

FROM base AS dev

EXPOSE 8000

CMD [ "cargo", "run", "--offline" ]