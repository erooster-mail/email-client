# Gtk4 Email Client

This is currently WIP.

## Building

### Rust

To just run it locally running `cargo run` is enough.

### Flatpak

To test a flatpak build you first need to run:

```bash
flatpak-builder --user flatpak_app build-aux/de.nordgedanken.Email.Devel.yaml --force-clean
```

to build it and then you can use the following command to launch it:

```bash
flatpak-builder --run flatpak_app build-aux/de.nordgedanken.Email.Devel.yaml mail-client
```
