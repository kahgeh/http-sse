# Background

Provides an example of how a http server sent event works

# Run

1. Start server `go run main.go`
2. Start webclient `cd webclient && npm start`
3. using curl or pwsh iwr, send request send a PUT send 
    e.g. in pwsh `iwr -method PUT http://localhost:8080/event`

