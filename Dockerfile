FROM rust:1.67

WORKDIR /app

RUN groupadd -r rust && useradd -r -g rust rust
RUN mkdir -p db && chown -R rust:rust db 

COPY src ./src
COPY  Cargo.toml .
COPY Cargo.lock .

RUN cargo install --path .

CMD ["add_to_notion_bot_oxid"]