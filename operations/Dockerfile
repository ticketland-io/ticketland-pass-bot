FROM rust:1.65 as builder

ARG GITHUB_TOKEN
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

RUN git config --global url."https://${GITHUB_TOKEN}@github.com/".insteadOf "https://github.com/"

RUN USER=root cargo new --bin ticketland-pass-bot-dist
COPY . ./ticketland-pass-bot-dist
WORKDIR ./ticketland-pass-bot-dist
RUN cargo build --release

RUN git config --global --unset url."https://${GITHUB_TOKEN}@github.com/".insteadOf

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq5 \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /ticketland-pass-bot-dist/target/release/ticketland-pass-bot ${APP}/ticketland-pass-bot

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

ENV ENV=production
CMD ["./ticketland-pass-bot"]
