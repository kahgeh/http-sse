package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

func handlePing() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Printf("received request")
		r.Context().Done()
	}
}

func handleConnect(messageChannel chan string, w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	flusher, _ := w.(http.Flusher)
	for {
		select {
		case message := <-messageChannel:
			fmt.Fprintf(w, "data: %s\n\n", message)
			flusher.Flush()

		case <-r.Context().Done():
			return
		}
	}
}

func handleSend(messageChannel chan string, w http.ResponseWriter, r *http.Request) {
	if messageChannel != nil {
		log.Printf("print message to client")
		messageChannel <- "done"
	}
}

func handleEvent() http.HandlerFunc {
	messageChannel := make(chan string)
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "PUT" {
			handleSend(messageChannel, w, r)
			return
		}
		if r.Method == "GET" {
			handleConnect(messageChannel, w, r)
			return
		}
		http.NotFound(w, r)
	}
}

func main() {
	log.Print("http.sse: starting server")
	http.HandleFunc("/ping", handlePing())
	http.HandleFunc("/event", handleEvent())
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	log.Printf("http.sse: listening on port %s", port)
	log.Fatal((http.ListenAndServe(fmt.Sprintf(":%s", port), nil)))
}
