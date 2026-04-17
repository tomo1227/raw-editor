FROM rust:1.95.0-trixie

USER root
ENV DEBIAN_FRONTEND=noninteractive

ARG USERNAME="vscode"
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        sudo findutils binutils pkg-config bash-completion \
        gcc tar wget make curl openssh-client locales \
        bash git gnupg2 ca-certificates vim tree \
        libx11-dev libgl1-mesa-dev libxcb1-dev libxrandr-dev \
        libxi-dev libxinerama-dev libxcursor-dev x11-apps libxkbcommon-x11-dev\
    && rm -rf /var/lib/apt/lists/*

RUN umask 0002

RUN groupadd --gid $USER_GID $USERNAME  && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME  && \
    echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME  && \
    chmod 0440 /etc/sudoers.d/$USERNAME

RUN chsh --shell /bin/bash ${USERNAME}

RUN echo 'export PS1="\[\033[0;32m\]\u\[\033[0m\] ➜ \[\033[1;34m\]\w\[\033[0m\] \$(if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then echo \"(\[\033[1;31m\]\$(git rev-parse --abbrev-ref HEAD)\[\033[0m\])\"; fi)\$ "' >> /home/$USERNAME/.bashrc

RUN echo "ja_JP.UTF-8 UTF-8" > /etc/locale.gen \
    && locale-gen \
    && update-locale LANG=ja_JP.UTF-8 LC_ALL=ja_JP.UTF-8

RUN rustup component add rustfmt clippy

ENV LANG=ja_JP.UTF-8
ENV LC_ALL=ja_JP.UTF-8

WORKDIR /workspace
USER $USERNAME

SHELL ["/bin/bash", "-c"]
CMD ["/bin/bash"]
