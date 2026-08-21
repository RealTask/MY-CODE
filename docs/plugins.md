# Plugins

MY CODE exposes a small in-process plugin registry (`my_code::plugins`) that
can record named plugins and whether they are enabled.

Custom tools and providers can be registered at runtime through that registry.
Plugin-specific configuration may also be stored under the `[plugins]` table
in `my-code.toml`.

```toml
[plugins.demo]
enabled = true
```

A stable WASM / dynamic-library ABI is not required for the current version;
plugins are compiled in or configured locally.
