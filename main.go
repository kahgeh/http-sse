package main

import (
	b64 "encoding/base64"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"sync"

	"github.com/gorilla/mux"
)

type event struct {
	clientID string
	payload  string
}

type safeChannel struct {
	clientID string
	value    chan event
	mutex    sync.Mutex
}

func (c *safeChannel) delete(clientChannelsMap map[string]*safeChannel) {
	if c == nil {
		return
	}
	c.mutex.Lock()
	defer c.mutex.Unlock()
	log.Printf("deleting channel %v...", c.clientID)
	close(c.value)
	delete(clientChannelsMap, c.clientID)
	log.Printf("deleting channel %v completed", c.clientID)
}

func (c *safeChannel) send(event event) {
	if c == nil {
		return
	}
	c.mutex.Lock()
	defer c.mutex.Unlock()
	c.value <- event
}

var clientChannels map[string]*safeChannel = make(map[string]*safeChannel)

func handlePing() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		log.Printf("received request")
	}
}

func sendEvent(w http.ResponseWriter, r *http.Request, clientID string) {
	clientChannels[clientID] = &safeChannel{
		clientID: clientID,
		value:    make(chan event),
		mutex:    sync.Mutex{},
	}
	eventChannel := clientChannels[clientID].value
	defer func() {
		clientChannels[clientID].delete(clientChannels)
	}()
	flusher, _ := w.(http.Flusher)
	fmt.Fprintf(w, "data: %s\n\n", "connected")
	flusher.Flush()
	for {
		select {
		case event := <-eventChannel:
			log.Printf("handler for %v: receiving event for %v", clientID, event.clientID)
			if event.clientID == clientID {
				fmt.Fprintf(w, "data: %s\n\n", event.payload)
				flusher.Flush()
				log.Printf("sending event to client %v", clientID)
			}

		case <-r.Context().Done():
			return
		}
	}
}
func handleConnect(w http.ResponseWriter, r *http.Request, clientID string) {
	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	sendEvent(w, r, clientID)
}

func handleSend(event event) {
	clientChannel := clientChannels[event.clientID]
	if clientChannel != nil {
		log.Printf("receive send event request to client %v", event.clientID)
		clientChannel.send(event)
	}
}

func handleEvent() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		vars := mux.Vars(r)
		clientID := vars["clientID"]
		if r.Method == "PUT" {
			payload, err := ioutil.ReadAll(r.Body)
			if err != nil {
				payload = []byte{}
			}
			event := event{clientID: clientID,
				payload: b64.StdEncoding.EncodeToString(payload)}
			handleSend(event)
			return
		}
		if r.Method == "GET" {
			handleConnect(w, r, clientID)
			return
		}
		http.NotFound(w, r)
	}
}

func main() {
	log.Print("http.sse: starting server")
	router := mux.NewRouter()
	router.HandleFunc("/ping", handlePing()).Methods("GET")
	router.HandleFunc("/clients/{clientID}/events", handleEvent())
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}
	log.Printf("http.sse: listening on port %s", port)

	log.Fatal((http.ListenAndServe(fmt.Sprintf(":%s", port), router)))
}
