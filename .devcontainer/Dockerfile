# The official rust images are based on Debian, but we need Ubuntu for eCAL
FROM ubuntu:22.04

# ECAL_VER=auto  -> resolve the newest release via the GitHub API
# ECAL_VER=<tag> -> use that tag (e.g. v6.0.0-rc.5) as-is
ARG ECAL_VER=auto
ENV ECAL_VER=${ECAL_VER}

RUN set -eux; \
    # Install dependencies for sharing GPG keys, the eCAL installation, rustecal and examples
    apt-get update; \
    DEBIAN_FRONTEND=noninteractive \
    apt-get install -y --no-install-recommends \
        gnupg2 \
        ca-certificates curl jq \
        clang libclang-14-dev llvm-dev \
        protobuf-compiler && \
    # Replace the placeholder with the actual eCAL version
    if [ "$ECAL_VER" = "auto" ]; then \
        ECAL_VER=$(curl -sSL https://api.github.com/repos/eclipse-ecal/ecal/releases \
            | jq -r '.[] | select((.draft|not) and (.prerelease|not)) | .tag_name' \
            | sort -V \
            | tail -n1); \
    fi; \
    # Locate the .deb that matches the tag and Ubuntu 22.04
    DEB_URL=$(curl -sSL https://api.github.com/repos/eclipse-ecal/ecal/releases/tags/${ECAL_VER} \
              | jq -r '[.assets[] | select(.name|test("jammy_amd64\\.deb$"))][0].browser_download_url'); \
    curl -sSL -o /tmp/ecal.deb "$DEB_URL" && \
    # Install eCAL and tidy up
    apt-get install -y /tmp/ecal.deb; \
    rm /tmp/ecal.deb; \
    rm -rf /var/lib/apt/lists/*
