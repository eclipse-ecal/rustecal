# The official rust images are based on Debian, but we need Ubuntu for eCAL
FROM ubuntu:22.04

# First: install dependencies for eCAL installation, rustecal and examples
# Then: install eCAL from the latest (pre-)release
# Finally: clean up
RUN set -eux && \
    apt-get update && \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends \
        ca-certificates curl jq pre-commit \
        clang libclang-14-dev llvm-dev \
        protobuf-compiler && \
    \
    DEB_URL=$( \
        curl -sSL https://api.github.com/repos/eclipse-ecal/ecal/releases | \
        jq -r '[ .[] | .assets[]? | select(.name|test("jammy_amd64\\.deb$")) ][0].browser_download_url' \
    ) && \
    curl -sSL -o ecal.deb "${DEB_URL}" && \
    apt-get install -y ./ecal.deb && \
    \
    rm ecal.deb && \
    rm -rf /var/lib/apt/lists/*
