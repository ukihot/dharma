ARG VARIANT=1-bullseye
FROM mcr.microsoft.com/devcontainers/rust:0-${VARIANT}

# 開発用ユーザ
ARG ID=20000
ARG USER_EMAIL
ARG USER_NAME
RUN groupadd -g ${ID} ${USER_NAME}
RUN useradd -m -s /bin/bash -u ${ID} -g ${ID} -G sudo ${USER_NAME}
RUN echo ${USER_NAME}:pass | chpasswd && \
  echo "${USER_NAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
USER ${USER_NAME}

# Git設定
RUN git config --global user.email ${USER_EMAIL}
RUN git config --global user.name ${USER_NAME}

# Tauriに必要なライブラリ
RUN sudo apt-get update
RUN sudo apt-get install -y libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# yarn を入れとく
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
RUN sudo apt-get update
RUN sudo apt install -y yarn

# node を最新化
RUN sudo yarn global add n
RUN sudo n latest

# Tauri をインストール
RUN cargo install tauri-cli

