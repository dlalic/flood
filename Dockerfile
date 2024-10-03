FROM rust:1.81-slim-bookworm AS builder

RUN apt-get update
RUN apt-get install -y curl
RUN curl -fsSL https://deb.nodesource.com/setup_22.x -o nodesource_setup.sh
RUN bash nodesource_setup.sh
RUN apt-get install -y nodejs

WORKDIR /usr/src/app
COPY . .

WORKDIR /usr/src/app/www
ENV NODE_ENV=production
RUN npm install
RUN npm run build

FROM nginx:1.19
COPY --from=builder /usr/src/app/www/default.conf.template /etc/nginx/conf.d/default.conf.template
COPY --from=builder /usr/src/app/www/nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /usr/src/app/www/dist /usr/share/nginx/html

CMD ["/bin/bash", "-c", "envsubst '\$PORT' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf"]
CMD ["nginx", "-g", "daemon off;"]