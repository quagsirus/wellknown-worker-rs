# wellknown-worker-rs
⚠️ This was just a small project written to get me back into Rust that got the job done at the time. Not production-ready.

Archiving since I don't use it any more and don't want to maintain it.

This is a [Cloudflare Worker](https://developers.cloudflare.com/workers/) project that exists so that the [.well-known](https://en.wikipedia.org/wiki/Well-known_URI) services [on my website](https://catpowered.net/.well-known/security.txt) remain accessible while my host server is down

## Features
- security.txt
- webfinger (only with very basic OIDC discovery functionality)
- ~~[OpenPGP Web Key Directory](https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/)~~ (only on `wkd` branch, and you should really stop and think about why you want this functionality)

## Customisation
### security.txt
Edit `src/security.txt`

### webfinger
Edit `src/webfinger_response.json` and replace the default resource URI in `src/handlers.rs`

## Building & running locally
1. Install the WASM rust target
```sh
rustup target add wasm32-unknown-unknown
```
2. [Install NPM](https://nodejs.org/en/download)
3. Build + start
```sh
npx wrangler dev
```

## Publishing to Cloudflare
```sh
npx wrangler deploy
```

