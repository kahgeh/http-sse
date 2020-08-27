# Background

Provides an example of how a http server sent event works

# Run

1. `docker-compose up` 
2. open up the browser at `http://localhost:8081` ( note down the client id), you can have multiple tabs
3. using curl or pwsh iwr, send request send a PUT send 
    e.g. in pwsh `iwr -method PUT http://localhost:8080/clients/<clientID shown on browser app>/events -Body "gooohhhhhhhhhhhhhhh"`