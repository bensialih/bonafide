# bonafide

basic udp application created to receive and consolidate logs.


## motivation

Trying to create a lightwight and specialized version of fail2ban that consumes a minimal amount of CPU and RAM.


## TODO
- [x] create api endpoint to receive log entries
- [ ] create endpoint that receives both single and batch logs
- [ ] pass api properties as env values
- [ ] Look to materlize for insperation on how data can be calculated with the lowewst latency.

- [ ] test bottlenecks
- [ ] benchmark


## setup

- git clone repo
- look at settings.toml and put in your own credentials
- create a .env file with postgres database credentials. This is loaded when docker compose is ran.
- docker-compose up in a separate termnial
- cargo run
- send udp packets to endpoint.
