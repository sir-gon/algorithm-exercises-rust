ARG BASE_IMAGE_VERSION=rust:1.91.0-alpine3.20
FROM ${BASE_IMAGE_VERSION} AS init

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

RUN apk add --update --no-cache make
RUN apk add --update --no-cache musl-dev
# cargo-chef to cache dependencies https://crates.io/crates/cargo-chef
RUN cargo install cargo-chef

COPY ./Makefile ${WORKDIR}/
COPY ./src ${WORKDIR}/src
COPY ./Cargo.lock ${WORKDIR}/Cargo.lock
COPY ./Cargo.toml ${WORKDIR}/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM init AS cacher

# COPY --from=init ${WORKDIR}/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

###############################################################################

FROM init AS builder

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

# sources
COPY ./src ${WORKDIR}/src

# Copy over the cached dependencies
COPY --from=cacher ${WORKDIR}/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

# build
RUN ls -alhR && \
  make build/debug && \
  make build/release && \
  ls -alhR

CMD ["make", "build"]

###############################################################################

FROM builder AS development

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

CMD []

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


CMD ["make", "lint"]

###############################################################################

FROM development AS testing

ENV WORKDIR=/app
WORKDIR ${WORKDIR}

RUN mkdir -p ${WORKDIR}/target
COPY --from=builder ${WORKDIR}/target/debug ${WORKDIR}/target

CMD ["make", "test"]

###############################################################################

FROM alpine:3.22 AS production

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
    "$USER" \
&&  ls -alhR

USER $USER

CMD ["make", "run"]
