FROM gcr.io/google-appengine/golang

ENV GOPATH /go
RUN mkdir -p /go/bin

COPY src /go/src

RUN go build -o /go/bin/secure-echo-server /go/src/secure-echo-server/cmd/secure-echo-server-server/main.go

CMD ["/go/bin/secure-echo-server","--host","0.0.0.0","--port","8080"]
