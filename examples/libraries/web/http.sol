import Server from "web/http"

Server
    .init("0.0.0.0:8099")
    .serve(fn (method, url) {
        println("Handling request for " + method + " at " + url + "...")

        return "Hello, world!"
    })