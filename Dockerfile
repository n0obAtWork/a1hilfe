FROM nginx:1.29.8-trixie

COPY nginx.conf /etc/nginx/nginx.conf

COPY book/html /usr/share/nginx/html