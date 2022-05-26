extern crate three;

use sol_compiler::compile;
use rquickjs::{BuiltinLoader, BuiltinResolver, FileResolver, Runtime, ModuleLoader, ScriptLoader, Context, Func, Value, Rest, bind};
use rustyline::{Editor, error::ReadlineError};
use structopt::StructOpt;

const VERSION: &str = "1.2.0";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "debug", short = "d", help = "Output debug information (JS, memory usage, etc)")]
    debug: bool,

    #[structopt(long = "raw", short = "r", help = "Execute the specified file as raw JavaScript")]
    raw: bool,

    #[structopt(long = "version", short = "v", help = "Output the current version of Sol.")]
    version: bool,

    file: Option<String>,
}

const POLYFILL: &str = include_str!("../js/polyfill.js");
const WEB_MODULE: &str = include_str!("../dist/web.js");
const JSON_MODULE: &str = include_str!("../js/json.js");

pub fn println(vs: Rest<Value>) {
    fn stringify(v: Value) -> String {
        match true {
            _ if v.is_string() => v.into_string().unwrap().to_string().unwrap(),
            _ if v.is_number() => v.as_number().unwrap().to_string(),
            _ if v.is_bool() => v.as_bool().unwrap().to_string(),
            _ if v.is_array() => v.into_array().unwrap().into_iter().map(|v| stringify(v.unwrap())).collect::<Vec<String>>().join(", "),
            _ => {
                unimplemented!()
            },
        }
    }

    for v in vs.into_inner().into_iter() {
        println!("{}", stringify(v));
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod token {
    use sol_compiler::{lex, TokenKind};

    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct Lexer {
        source: String,
    }

    impl Lexer {
        pub fn new(source: String) -> Self {
            Self {
                source
            }
        }

        pub fn all(&self) -> Vec<(String, String)> {
            let tokens = lex(&self.source[..]);
            let mut js = Vec::new();

            for token in tokens {
                js.push(match token.kind {
                    TokenKind::Identifier(s) => (s, "Identifier".to_owned()),
                    TokenKind::String(s) => (s, "String".to_owned()),
                    TokenKind::Number(n) => (n.to_string(), "Number".to_owned()),
                    TokenKind::Fn => ("fn".to_owned(), "Fn".to_owned()),
                    TokenKind::LeftParen => ("(".to_string(), "LeftParen".to_owned()),
                    TokenKind::RightParen => (")".to_string(), "RightParen".to_owned()),
                    TokenKind::LeftBracket => ("[".to_string(), "LeftBracket".to_owned()),
                    TokenKind::RightBracket => ("]".to_string(), "RightBracket".to_owned()),
                    TokenKind::LeftBrace => ("{".to_string(), "LeftBrace".to_owned()),
                    TokenKind::RightBrace => ("}".to_string(), "RightBrace".to_owned()),
                    _ => unimplemented!("{:?}", token.kind)
                });
            }

            js
        }

        pub fn tokenize(source: String) -> Vec<(String, String)> {
            let this = Self::new(source);

            this.all()
        }
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod fs {
    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct File {
        path: String,
        contents: String,
    }

    impl File {
        pub fn new(path: String) -> Self {
            // TODO: Check the file exists before trying to read it.
            Self {
                path: path.clone(),
                contents: std::fs::read_to_string(path.trim()).unwrap(),
            }
        }

        pub fn path(&self) -> String {
            self.path.clone()
        }

        pub fn lines(&self) -> Vec<&str> {
            self.contents.lines().collect()
        }

        pub fn is_empty(&self) -> bool {
            self.contents.is_empty()
        }

        pub fn exists(path: String) -> bool {
            std::fs::metadata(path).is_ok()
        }

        pub fn contents(&self) -> String {
            self.contents.clone()
        }

        pub fn read(path: String) -> Self {
            Self::new(path)
        }
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod math {
    #[quickjs(bare)]
    pub struct Math {
        pub pi: f64,
        pub e: f64,
        pub tau: f64,
    }

    impl Math {
        pub fn new() -> Self {
            Self {
                pi: 3.141592653589793,
                e: 2.718281828459045,
                tau: 6.283185307179586,
            }
        }

        // Constants

        pub fn pi() -> f64 {
            std::f64::consts::PI
        }

        pub fn e() -> f64 {
            std::f64::consts::E
        }

        pub fn tau() -> f64 {
            std::f64::consts::TAU
        }

        // Trigonometry functions

        pub fn acos(x: f64) -> f64 {
            (x.acos() * 180.0) / std::f64::consts::PI
        }

        pub fn asin(x: f64) -> f64 {
            (x.asin() * 180.0) / std::f64::consts::PI
        }

        pub fn atan(x: f64) -> f64 {
            (x.atan() * 180.0) / std::f64::consts::PI
        }

        pub fn atan2(y: f64, x: f64) -> f64 {
            (x.atan2(y) * 180.0) / std::f64::consts::PI
        }

        pub fn cos(x: f64) -> f64 {
            x.cos()
        }

        pub fn sin(x: f64) -> f64 {
            x.sin()
        }

        pub fn tan(x: f64) -> f64 {
            x.tan()
        }

        // Hyperbolic functions
        
        pub fn acosh(x: f64) -> f64 {
            x.acosh()
        }

        pub fn asinh(x: f64) -> f64 {
            x.asinh()
        }

        pub fn atanh(x: f64) -> f64 {
            x.atanh()
        }

        pub fn cosh(x: f64) -> f64 {
            x.cosh()
        }

        pub fn sinh(x: f64) -> f64 {
            x.sinh()
        }

        pub fn tanh(x: f64) -> f64 {
            x.tanh()
        }

        // Exponential and logarithmic functions

        pub fn exp(x: f64) -> f64 {
            x.exp()
        }

        pub fn expm1(x: f64) -> f64 {
            x.exp_m1()
        }

        pub fn exp2(x: f64) -> f64 {
            x.exp2()
        }

        pub fn ln(x: f64) -> f64 {
            x.ln()
        }

        pub fn log10(x: f64) -> f64 {
            x.log10()
        }

        pub fn log2(x: f64) -> f64 {
            x.log2()
        }

        // Power functions

        pub fn pow(x: f64, y: f64) -> f64 {
            x.powf(y)
        }

        pub fn sqrt(x: f64) -> f64 {
            x.sqrt()
        }

        pub fn cbrt(x: f64) -> f64 {
            x.cbrt()
        }

        pub fn hypot(x: f64, y: f64) -> f64 {
            x.hypot(y)
        }

        // Rounding, remainder and other functions

        pub fn ceil(x: f64) -> f64 {
            x.ceil()
        }

        pub fn floor(x: f64) -> f64 {
            x.floor()
        }

        pub fn trunc(x: f64) -> f64 {
            x.trunc()
        }

        pub fn round(x: f64) -> f64 {
            x.round()
        }

        pub fn abs(x: f64) -> f64 {
            x.abs()
        }

        pub fn sign(x: f64) -> f64 {
            x.signum()
        }

        pub fn fract(x: f64) -> f64 {
            x.fract()
        }

        pub fn min(x: f64, y: f64) -> f64 {
            if x < y {
                x
            } else {
                y
            }
        }

        pub fn max(x: f64, y: f64) -> f64 {
            if x > y {
                x
            } else {
                y
            }
        }

        pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
            x.clamp(min, max)
        }

        pub fn random() -> f64 {
            rand::random()
        }
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod env {
    use std::env::{var};

    pub fn get(name: String) -> String {
        match var(name) {
            Ok(value) => value,
            Err(_) => unreachable!()
        }
    }

    pub fn has(name: String) -> bool {
        var(name).is_ok()
    }
}

#[bind(module, public)]
#[quickjs(bare)]
mod uuid {
    use uuid::Uuid as UuidGenerator;

    #[derive(Clone)]
    #[quickjs(cloneable)]
    pub struct Uuid {
        value: String
    }

    impl Uuid {
        pub fn new() -> Self {
            Self {
                value: UuidGenerator::new_v4().to_string()
            }
        }

        pub fn to_string(&self) -> String {
            self.value.clone()
        }

        pub fn generate() -> String {
            UuidGenerator::new_v4().to_string()
        }
    }
}

fn main() {
    let args = Cli::from_args();

    if args.version {
        println!("Sol v{}", VERSION);
        
        std::process::exit(0);
    }

    let runtime: Runtime = Runtime::new().unwrap();
    runtime.set_max_stack_size(256 * 2048);

    let resolver = (
        BuiltinResolver::default()
            // File system module
            .with_module("fs")
            // Operating System modules
            .with_module("os/env")
            // Miscellaneous modules
            .with_module("misc/token")
            .with_module("misc/uuid")
            // Web modules
            .with_module("web/http")
            .with_module("web/website")
            .with_module("web/json")
            // Math module
            .with_module("math"),
        FileResolver::default()
            .with_path("./"),
    );

    let loader = (
        BuiltinLoader::default()
            // Web modules
            .with_module("web/website", WEB_MODULE)
            .with_module("web/json", JSON_MODULE),
        ModuleLoader::default()
            // File system module
            .with_module("fs", Fs)
            // Operating System modules
            .with_module("os/env", Env)
            // Miscellaneous modules
            .with_module("misc/uuid", Uuid)
            .with_module("misc/token", Token)
            // Web modules
            .with_module("web/http", Http)
            // Math module
            .with_module("math", Math),
        ScriptLoader::default(),
    );

    runtime.set_loader(resolver, loader);

    let context: rquickjs::Context = Context::full(&runtime).unwrap();
    
    if let Some(file) = args.file {
        let contents = read(file.clone());
        let compiled = [
            POLYFILL.to_string(),
            if args.raw { contents } else { compile(&contents[..]) }
        ].join("\n");

        let fqp = std::fs::canonicalize(file.clone()).unwrap();
        let fqd = fqp.parent().unwrap();
    
        if args.debug {
            println!("=== JS OUTPUT ===");
            println!("{}", compiled);
        }

        context.with(|ctx: rquickjs::Ctx| {
            let glob = ctx.globals();
    
            // Printing to le console
            glob.set("println", Func::from(println)).unwrap();
            // File system
            glob.set("__FILE__", fqp.to_str()).unwrap();
            glob.set("__DIR__", fqd.to_str()).unwrap();
    
            if args.debug {
                println!("=== EVAL ===");
            }
            
            ctx.compile(file, compiled).unwrap();
        });
    
        if args.debug {
            println!("=== DEBUG ===");
            println!("Memory used (bytes): {}", runtime.memory_usage().memory_used_size);
        }
    } else {
        println!("Sol v{} | Copyright (c) 2021-2022 Joshua Colell", VERSION);
        
        let mut rl = Editor::<()>::new();

        loop {
            let line = rl.readline("> ");

            match line {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    context.with(|ctx: rquickjs::Ctx| {
                        let glob = ctx.globals();
                
                        glob.set("println", Func::from(println)).unwrap();
                
                        ctx.eval::<(), _>(line).unwrap();
                    });
                },
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    println!("Exiting Sol...");
                    break
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    break
                }
            }
        }
    }
}

fn read(path: String) -> String {
    std::fs::read_to_string(path).unwrap()
}