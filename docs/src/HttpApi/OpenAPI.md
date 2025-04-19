# OpenAPI definition

BulletForceHax provides documentation of the Bullet Force HTTP API surface in the form of an [OpenAPI definition][openapispec] with accompanying [Swagger UI][swagger-ui] interface. BulletForceHax developers maintain this OpenAPI definition on a best-effort basis, meaning it may not be up to date or may miss various significant endpoints.

## Generating OpenAPI clients

One use for OpenAPI definitions is to automatically generate API clients. These clients allow programmatic access to the API through a more high-level API with code that's generated at build-time.

The Bullet Force API is problematic in this regard because it returns a `text/html` content-types for JSON-formatted responses. OpenAPI client generators can't reasonably predict this, and some use a generic byte stream or string response type instead. You can work around this in several ways:

- Modify the OpenAPI definition to specify `application/json` responses instead. This tricks the client generator into generating JSON parsing code, although this causes problems with generated clients that check the response's return type at runtime.
- Manually parse the returned byte stream or string into the correct data type. The OpenAPI definition still contains all return types, so client generators should include those models in their generate code too.

## OpenAPI definition

Below is a copy of the full OpenAPI definition as it was while during compilation of this book. You can also find this [hosted online][openapispec] or in the GitHub repository.

```yaml
{{ #include ../../openapi.yml }}
```


[openapispec]: https://variant9.dev/BulletForceHaxV3/openapi.yml
[swagger-ui]: https://variant9.dev/BulletForceHaxV3/swagger-ui/
