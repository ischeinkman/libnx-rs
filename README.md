# libnx-rs

## Description

Raw bindings to [libnx](https://github.com/switchbrew/libnx) to be used from Rust for the purpose of making Switch homebrew in Rust.

Note that the API is currently unstable; at the moment all function and struct bindings are generated at final compile time in `libnx_rs::libnx` via bindgen, but this may change.

For information on how to actually *use* this library, see [libnx-rs-template](https://github.com/ischeinkman/libnx-rs-template).

## Credits and Thanks

* [Igor1201's rusted-switch](https://github.com/Igor1201/rusted-switch) which provided the foundation for this template.

* [Roblabla and his mighty Megaton Hammer](https://github.com/MegatonHammer/megaton-hammer) for allowing me to take over his Discord server.