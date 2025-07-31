# Warframe helper

This is a simple helper site for warframe.

## Build 

### build ui

```bash
$ cd ui
$ npm install
$ npm run build
```

### build api

```bash
$ cargo build --release
```

### bundle ui with api

first you should complete build ui step above.

```bash
$ cargo build --release --feature embed
```

## TODO

- [ ] CLI for generating configurations and restore admin credentials
- [ ] Warframe world state api
- [ ] User management api (create new user, delete user, change password, change role)
- [ ] User management web interface
- [ ] Translations for mods
- [ ] User inventory
