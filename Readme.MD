# Background

Provides an example of how a http server sent event works

# Run

1. `docker-compose up` 
2. open up the browser at `http://localhost:8081`
3. using curl or pwsh iwr, send request send a PUT send 
    e.g. in pwsh `iwr -method PUT http://localhost:8080/event`

