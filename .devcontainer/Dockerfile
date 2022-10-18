FROM quay.io/brainblock/rust-torch-cuda116

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# adding contrib and non-free
RUN sed -r -i 's/^deb(.*)$/deb\1 non-free/g' /etc/apt/sources.list &&\
    sed -r -i 's/^deb(.*)$/deb\1 contrib/g' /etc/apt/sources.list

# adding nvidia container toolkit deps
RUN distribution=$(. /etc/os-release;echo $ID$VERSION_ID) \
      && curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
      && curl -s -L https://nvidia.github.io/libnvidia-container/$distribution/libnvidia-container.list | \
            sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
            tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && apt-get update \
    && apt-get install -y nvidia-docker2 nvtop sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

