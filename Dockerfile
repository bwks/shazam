FROM alpine:3.16.2 as builder

WORKDIR /opt/

RUN apk add curl protoc musl-dev gzip git

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
RUN curl -sLO https://github.com/bwks/shazam/releases/latest/download/shazam-x86_64-unknown-linux-musl.tar.gz \
  && tar -xvf shazam-x86_64-unknown-linux-musl.tar.gz \
  && chmod +x shazam

#########################################################

FROM alpine:3.16.2

RUN apk add tmux

COPY --from=builder /opt/shazam /opt/shazam
COPY --from=builder /opt/tailwindcss /opt/tailwindcss
COPY --from=builder /opt/reflex /opt/reflex
COPY --from=builder /opt/overmind /opt/overmind

RUN chown -R root:root /opt/
