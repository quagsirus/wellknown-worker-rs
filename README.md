# ⚠️ WHY ARE YOU USING PGP
### "[entity] requires me to, and I can't ask them to use something else" is the *only* good reason
seriously, please think about using something else.

See:
- [What To Use Instead of PGP](https://soatok.blog/2024/11/15/what-to-use-instead-of-pgp/) (Soatok)
- [The PGP problem](https://www.latacora.com/blog/2019/07/16/the-pgp-problem/) (Latacora)

# wellknown-worker-rs
⚠️ Horrible code quality ahead, this is just a small project written to get me back into Rust that gets the job done well-enough for my personal site. Not production-ready.

This is a [Cloudflare Worker](https://developers.cloudflare.com/workers/) project that exists so that the [.well-known](https://en.wikipedia.org/wiki/Well-known_URI) services on my website remain accessible while my host server is down

## Features
- security.txt
- webfinger (only with very basic OIDC discovery functionality)
- [OpenPGP Web Key Directory](https://datatracker.ietf.org/doc/draft-koch-openpgp-webkey-service/) (only on `wkd` branch, and you should really stop and think about why you want this functionality)

## Customisation
### security.txt
Edit `src/security.txt`

### webfinger
Edit `src/webfinger_response.json` and replace the default resource URI in `src/handlers.rs`

### OpenPGP Web Key Directory
1. Create a new folder in the project root, `pgp_keys`
2. Place all keys you want to host within it, in binary format (e.g. `pgp_keys/my_awesome_key.pgp`)
3. They'll be included when you next build & publish

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

