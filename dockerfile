FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    zip \
    unzip \
    curl \
    tree \
    colordiff \
    vim \
    tmux \
    bash-completion \
    htop \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/cpgrader-rs /usr/src/app/

WORKDIR /usr/src/app

RUN if [ -d ./example ]; then cp -r ./example ./example; fi
RUN if [ -d ./testcase ]; then cp -r ./testcase ./testcase; fi
RUN if [ -f ./config.toml ]; then cp ./config.toml ./config.toml; fi
RUN mkdir /usr/src/app/grader

RUN echo "source /etc/profile.d/bash_completion.sh" >> ~/.bashrc

CMD ["/bin/bash"]