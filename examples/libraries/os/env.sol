import get, has from "os/env"

println(get("Path"))
println(has("NON_EXISTENT_ENV_VAR"))