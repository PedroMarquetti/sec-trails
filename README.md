# Welcome to my URL-Info-getter script

## Why?

I wanted to learn Rust, so I decided to work on this simple script... I could easily do this in Python or JS, but challenging yourself is more fun hehe.

## How it works?

This script fetches data from [Security Trails](https://securitytrails.com/)'s [API](https://docs.securitytrails.com/docs) to get some info about the specified **URL** (you **MUST** specify your API Key - _it's free yayy_).

## What this program currently do?

For now, it only prints the current_dns 'A record' ip and ip_organization field (at index 0) that the API sent:

```
"current_dns": {
    "a": {
        "values": [
        {
            "ip": "<IP Addres of requested URL",
            "ip_organization": "ip_organization name"
        }
    ]
}
```

In the future, you'll be able to get more fields (like DNS records, details and so on).

## How to run

If you have [rustup](https://rustup.rs/) installed, you can simply run

`cargo run -- -k <YOUR KEY> -u <URL>`

This will build the program in debug mode, creating a /target dir
