import JSON from "web/json"

let person = {
    "name": "User",
    "age": 21,
    "email": "user@sol.com",
}

let encoded = JSON.encode(person)

println("Encoded:", encoded)

let decoded = JSON.decode(encoded)

println([decoded.name, decoded.age, decoded.email])