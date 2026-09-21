# Warframe helper

This is a simple helper site for warframe.

## Dependencies

This library binds to `liboo2corelinux64.so` on Linux and `oo2core_win64.lib` on Windows. MacOS isn't supported. You can retrieve it by following the instructions here : https://github.com/sehnryr/get-oodle-lib

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
$ cargo build --release --features embed
```

### Run

```bash
$ ./target/release/wf_api serve
```

<!-- TODO: ADD MORE DOCUMENT -->

## TODO

- [X] CLI for generating configurations
- [X] Warframe world state api
- [X] World state ui
- [ ] User management api (create new user, delete user, change password, change role)
- [ ] User management web interface
- [ ] Translations for mods
- [ ] User inventory
- [ ] CLI for restore admin credentials