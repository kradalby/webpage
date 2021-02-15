FROM nginx:alpine

COPY redirect_pelican.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /usr/share/nginx/html
