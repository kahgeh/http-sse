FROM node:14 as builder
WORKDIR /src
COPY *.json ./
COPY index.html ./
RUN npm i
COPY *.ts* ./
EXPOSE 1234
