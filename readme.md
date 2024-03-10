This is my fork of [bevy_mod_scripting](https://github.com/makspll/bevy_mod_scripting) -- see its README for more information.

## My Changes

- Just version stuff to work with bevy (hopefully with latest version).

## How To

### Generating bindings (generated.rs)

- `cargo run bevy_script_api`
  - Install the nightly version if you need to `rustup install nightly-2023-07-16`.
- `make make_json_files`
  - May need to update Makefile versions and retry.
- `make generate_api`