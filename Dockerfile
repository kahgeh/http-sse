FROM golang:1.15 as builder
WORKDIR /app

COPY go.* ./
RUN go mod download

COPY . ./
RUN CGO_ENABLED=0 GOOS=linux go build -mod=readonly -v -o server
FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=builder /app/server /server
ARG PORT=80
ENV PORT ${PORT}
ARG SERVICENAME=sse
ENV URLPREFIX="/${SERVICENAME}"

LABEL CLUSTER_${PORT}_NAME=${SERVICENAME}

EXPOSE ${PORT}

CMD ["/server"]