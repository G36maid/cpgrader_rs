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

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

RUN cp /usr/src/myapp/target/release/cpgrader-rs .

RUN echo "source /etc/profile.d/bash_completion.sh" >> ~/.bashrc
# 進入 shell
CMD ["/bin/bash"]