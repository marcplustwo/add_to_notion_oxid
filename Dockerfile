FROM rust:1.67

WORKDIR /app
COPY src ./src
COPY  Cargo.toml .
COPY Cargo.lock .

RUN cargo install --path .

CMD ["add_to_notion_bot_oxid"]