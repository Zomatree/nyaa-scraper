# Nyaa RSS Scraper

A very simple nyaa rss scaper to post to a discord webhook

![ayaya](https://www.streamscheme.com/wp-content/uploads/2020/10/ayaya-emote.png)

## Running
```bash
$ git clone https://github.com/zomatree/nyaa-scraper
$ cd nyaa-scraper
$ cargo build --release
$ cp config.example.toml config.toml
```

Edit the `config.toml` to point to the correct urls

move `target/release/nyaa_scraper` somewhere and run it to start the program
