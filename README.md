# Learn to define API Gateway and Lambda using Rust
[![Rust CI](https://github.com/yoshitaka-motomura/learn-aws-rust/actions/workflows/rust_ci.yml/badge.svg?branch=main)](https://github.com/yoshitaka-motomura/learn-aws-rust/actions/workflows/rust_ci.yml)

## Description

This is a sample project to learn how to define API Gateway and Lambda using Rust.
It is for personal use only.

:flag_japan: I speak Japanese. Therefore, Japanese may be mixed in.

## Requirement
- Rust [install](https://www.rust-lang.org/tools/install)
- AWS CLI [install](https://docs.aws.amazon.com/cli/latest/userguide/uninstall.html)
- node.js(for stable) [install](https://nodejs.org/en/download/)
- cargo-lambda [install](https://www.cargo-lambda.info/guide/getting-started.html)

## Installed packages

```bash
cd cdk && npm install 
cp .env.sample .env // Your AWS account and region are required
```

## How to build

```bash
cd rust_lambda 
cargo lambda build
```

## How to deploy

Describe how to deploy to AWS.  

1. **If you have updated the Lambda function, you must first build it.**
```bash
cd rust_lambda
cargo lambda build
```
2. **Run deploy with CDK commands**
```bash
cd cdk
cdk deploy
```
see [cdk/README.md](cdk/README.md)

If you are detail about  aws cdk and Cargo-lambda for visit Documents here

- [aws cdk](https://docs.aws.amazon.com/cdk/latest/guide/home.html)
- [Cargo-lambda](https://www.cargo-lambda.info/guide/getting-started.html)

```bash
cd cdk
// cdk bootstrap // Run it when deploying for the first time
cdk deploy
```

## Directory structure

```bash
.
├── README.md
├── cdk
│   ├── README.md
│   ├── bin
│   │   ├── cdk.d.ts
│   │   ├── cdk.js
│   │   └── cdk.ts // cdk main
│   ├── cdk.json
│   ├── jest.config.js
│   ├── lib
│   │   ├── cdk-stack.d.ts
│   │   ├── cdk-stack.js
│   │   └── cdk-stack.ts // stack 
│   ├── package-lock.json
│   ├── package.json
│   ├── test
│   │   ├── cdk.test.d.ts
│   │   ├── cdk.test.js
│   │   └── cdk.test.ts
│   └── tsconfig.json
└── rust_lambda
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    └── src
        └── hello_lambda // lambda function
            ├── lib.rs
            └── main.rs

```
## learned
- [x] How to define API Gateway and Lambda using Rust
- [x] How to deploy using CDK
- [x] How to use Cargo-lambda

### Note
:warning: When using reqwest crate with cargo-lambda, the dependent environment `openssl-sys` causes an error (on Apple M1?)
Let's specify the clay as follows.

```toml
[dependencies]
reqwest = { version="0.11.20", default-features = false, features = ["json", "rustls-tls"]}

```

## Author

[@yoshitaka_motomura](https://twitter.com/yoshi_motomura)