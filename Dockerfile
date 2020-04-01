FROM nginx:alpine

COPY redirect_pelican.conf /etc/nginx/conf.d/default.conf
