FROM rust:1.64-alpine AS builder

WORKDIR /opt/shazam/

RUN apk add curl protoc musl-dev gzip

COPY . .

# tailwind
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
  && chmod +x tailwindcss-linux-x64 \
  && mv tailwindcss-linux-x64 tailwindcss

# reflex
RUN curl -sLO https://github.com/cespare/reflex/releases/latest/download/reflex_linux_amd64.tar.gz \
  && tar -xvf reflex_linux_amd64.tar.gz \
  && chmod +x reflex_linux_amd64/reflex \
  && mv reflex_linux_amd64/reflex reflex

# overmind
RUN curl -sLO https://github.com/DarthSim/overmind/releases/latest/download/overmind-v2.3.0-linux-amd64.gz \
  && gunzip overmind-v2.3.0-linux-amd64.gz \
  && chmod +x overmind-v2.3.0-linux-amd64 \
  && mv overmind-v2.3.0-linux-amd64 overmind

# shazam
RUN cargo build --release

#########################################################

FROM alpine:3.16.2

ARG APP_NAME
ARG APP_USER
ARG APP_USER_ID
ARG APP_GROUP_ID
ARG HOME_DIR
ENV APP_NAME ${APP_NAME}
ENV APP_USER ${APP_USER}
ENV APP_USER_ID ${APP_USER_ID}
ENV APP_GROUP_ID ${APP_GROUP_ID}
ENV HOME_DIR ${HOME_DIR}

RUN apk add tmux

# Create app user and group
RUN addgroup -S ${APP_USER} -g ${APP_GROUP_ID}  && adduser -u ${APP_USER_ID} -S ${APP_USER} -G ${APP_USER} -s /bin/ash

COPY --from=builder /opt/shazam/target/release/shazam ${HOME_DIR}/shazam
COPY --from=builder /opt/shazam/tailwindcss ${HOME_DIR}/tailwindcss
COPY --from=builder /opt/shazam/reflex ${HOME_DIR}/reflex
COPY --from=builder /opt/shazam/overmind ${HOME_DIR}/overmind

# Set directory ownership
RUN chown -R ${APP_USER_ID}:${APP_GROUP_ID} ${HOME_DIR}

WORKDIR ${HOME_DIR}

USER ${APP_USER}

RUN ./shazam init ${APP_NAME}

EXPOSE 3000
CMD ["./overmind", "s", "-f", "Procfile"]