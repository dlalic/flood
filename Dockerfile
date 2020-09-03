FROM rust:1.46-buster AS builder

RUN apt-get update
RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -
RUN apt-get install -y --no-install-recommends nodejs

WORKDIR /usr/src/app
COPY . .

RUN cargo install wasm-pack
RUN wasm-pack build --release

WORKDIR /usr/src/app/www
ENV NODE_ENV=production
RUN npm install
RUN npm run build --verbose

FROM nginx:1.19
COPY --from=builder /usr/src/app/www/default.conf.template /etc/nginx/conf.d/default.conf.template
COPY --from=builder /usr/src/app/www/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/app/www/dist /usr/share/nginx/html

CMD /bin/bash -c "envsubst '\$PORT' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf" && nginx -g 'daemon off;'
