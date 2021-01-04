FROM gitpod/workspace-full

USER gitpod

RUN sudo apt-get install -y libsdl2-dev
RUN sudo apt-get install -y libsdl2-ttf-dev
RUN rustup component add clippy
RUN rustup component add rustfmt

# Install custom tools, runtime, etc. using apt-get
# For example, the command below would install "bastet" - a command line tetris clone:
#
# RUN sudo apt-get -q update && \
#     sudo apt-get install -yq bastet && \
#     sudo rm -rf /var/lib/apt/lists/*
#
# More information: https://www.gitpod.io/docs/config-docker/
