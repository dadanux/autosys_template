FROM nginx
COPY ./dist /usr/share/nginx/html
COPY nginx /etc/nginx/conf.d
EXPOSE 9080
CMD ["nginx", "-g", "daemon off;"]