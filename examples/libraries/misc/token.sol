import Lexer from "misc/token"

let tokens = Lexer.tokenize("fn get() {}")

println(tokens)