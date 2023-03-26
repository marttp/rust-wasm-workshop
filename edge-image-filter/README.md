# Empty Starter Kit for Rust

An empty project template for Rust Compute@Edge programs that returns an empty `200 OK` response for
any request it receives.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.

## Command

```bash
mkdir edge-image-filter
cd edge-image-filter
fastly compute init
```

Give it a name of your choice.

When asked for the language to use select "Rust".

When asked for the Starter kit, use "[5] Empty starter for Rust".

```bash
fastly compute serve
```

Your application should be reachable at http://127.0.0.1:7676/.