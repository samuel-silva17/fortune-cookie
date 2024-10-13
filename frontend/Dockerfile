FROM docker.io/library/nginx:alpine

# Remove default nginx website
RUN rm -rf /usr/share/nginx/html/*

# Copy the HTML file to nginx directory
COPY index.html /usr/share/nginx/html

# Copy custom nginx config
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
