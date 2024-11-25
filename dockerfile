FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential \
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

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/app

COPY ./src/ ./src/
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN cp target/release/cpgrader-rs /usr/src/app/

COPY ./example/ ./example/
COPY ./testcase/ ./testcase/
COPY ./config.toml ./config.toml
RUN mkdir /usr/src/app/grader

RUN echo "source /etc/profile.d/bash_completion.sh" >> ~/.bashrc

CMD ["/bin/bash"]