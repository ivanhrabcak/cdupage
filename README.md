# cdupage
Cdupage's goal is to replace edupage_api and/or all other edupage libraries, so everyone can have a good experience. If someone wants to use edupage's api (cdupage) in a new language, they simply need to generate bindings, they do not need to rewrite the whole thing in the new language (as it was before).

# JS/TS
- Install Dependencies:
```shell
cargo install nj-cli
```
 
- Build node bindings 
```shell
nj-cli build --release -- --features node
```
Bindings will be generated in the `dist` directory
 
- Generate typescript types
```shell
cargo test --features node-types
```
Ignore any failed tests. The types will be generated in the `bindings` directory

# C/C++ (experimental)
Binds will be generated in the `bindings/` directory.
```shell
cargo build
```
# TODO (goals)
- [ ] All functionality from [edupage-api](https://github.com/ivanhrabcak/edupage-api) and [EdupageAPI](https://github.com/loumadev/EdupageAPI)
- [ ] Bindings for major languages (published, official)
- [ ] Publish on crates.io
- [ ] Proper documentation & README + Issue Templates

Feel free to [Open a Pull Request!](https://github.com/ivanhrabcak/cdupage/compare)