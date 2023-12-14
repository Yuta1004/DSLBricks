FROM node:21-alpine

WORKDIR /workdir

COPY package.json /workdir

RUN apk update && \
    npm cache verify && \
    npm install
