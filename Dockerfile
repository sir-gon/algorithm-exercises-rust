ARG BASE_IMAGE_VERSION=rust:1.89.0-alpine3.20
FROM ${BASE_IMAGE_VERSION} AS init

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

RUN apk add --update --no-cache make

# cargo-chef to cache dependencies https://crates.io/crates/cargo-chef

###############################################################################

FROM init AS builder

RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef

# sources
COPY ./src ${WORKDIR}/src
COPY ./Cargo.lock ${WORKDIR}/Cargo.lock
COPY ./Cargo.toml ${WORKDIR}/Cargo.toml
COPY ./Makefile ${WORKDIR}/

# dependencies
RUN make dependencies

# build
RUN ls -alhR && \
  make build/debug && \
  make build/release && \
  ls -alhR

CMD ["make", "build"]

###############################################################################

FROM builder AS development

# CMD []

CMD ["make", "build"]

###############################################################################

FROM builder AS lint

RUN rustup component add clippy
RUN \
  apk add --update --no-cache nodejs npm yamllint && \
  npm install -g --ignore-scripts markdownlint-cli && \
  rm -rf /var/lib/apt/lists/*

# Code source
COPY ./src ${WORKDIR}/src
COPY ./Cargo.lock ${WORKDIR}/Cargo.lock
COPY ./Cargo.toml ${WORKDIR}/Cargo.toml
COPY ./recipe.json ${WORKDIR}/recipe.json
COPY ./Makefile ${WORKDIR}/

# markdownlint conf
COPY ./.markdownlint.yaml ${WORKDIR}/

# yamllint conf
COPY ./.yamllint ${WORKDIR}/
COPY ./.yamlignore ${WORKDIR}/
COPY ./.gitignore ${WORKDIR}/

RUN cargo chef cook --release --recipe-path recipe.json

CMD ["make", "lint"]

###############################################################################

FROM development AS testing

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

RUN mkdir -p ${WORKDIR}/target
COPY --from=builder ${WORKDIR}/target/debug ${WORKDIR}/target

CMD ["make", "test"]

###############################################################################

FROM alpine:3.20 AS production

ENV LOG_LEVEL=INFO
ENV BRUTEFORCE=false
ENV WORKDIR=/app
WORKDIR ${WORKDIR}

RUN apk add --update --no-cache make

COPY ./Makefile ${WORKDIR}/
COPY --from=builder ${WORKDIR}/target/release/lib*.rlib ${WORKDIR}/
# COPY --from=builder ${WORKDIR}/target/release/lib*.dylib ${WORKDIR}/

# create a non-root user to run our application
ENV USER=app
ENV GROUPNAME=$USER
ENV UID=12345
ENV GID=23456

RUN addgroup \
    --gid "$GID" \
    "$GROUPNAME" \
&&  adduser \
    --disabled-password \
    --gecos "" \
    --home "$(pwd)" \
    --ingroup "$GROUPNAME" \
    --no-create-home \
    --uid "$UID" \
    $USER

RUN ls -alhR

CMD ["make", "run"]
