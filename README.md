# Learn to define API Gateway and Lambda using Rust

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

```bash
cd cdk
// cdk bootstrap // Run it when deploying for the first time
cdk deploy
```
