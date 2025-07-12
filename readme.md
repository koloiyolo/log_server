# log_server

## About

This repository contains log management server written in Rust.\
This project is part of my Rust learning journey. It aims to teach me basic concepts of\
stack I'll be using in my future work, which consists of:
* [x] [rust](https://doc.rust-lang.org/book/)
* [x] [tokio](https://docs.rs/tokio/latest/tokio/)
* [x] [nats.io](https://nats.io)
* [ ] [poem](https://docs.rs/poem/latest/poem/)
* [ ] [poem_openapi](https://docs.rs/poem-openapi/latest/poem_openapi/)

I've chosen the topic of logs server because it requires handling quick data flow, usually from multiple sources, which is similar in a way to what I'll be doing in my future work. I also have some expertise in this topic due to my diploma project which was *Project and implementation of log management and device monitoring system* available [here](https://github.com/koloiyolo/engineering_thesis_django)

## Current state

For now the log server has:
* [x] Cli interface
* [x] Fetch server
* [x] Processing server
* [ ] Database
* [ ] Api endpoints
* [ ] Api docs

Rest of docs is `todo!()`
