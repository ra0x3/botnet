# hello-world

A simple example of how `Botnet` can be plugged into a web server as a middleware.

## Usage

Run server.

```
RUST_LOG=debug cargo run -- -c config.yaml
```

```text
2023-08-09T00:07:56.591330Z  INFO hello_world: 73: Botnet { keys: {3472328297305896040: BotnetKey { metadata: BotnetKeyMetadata { field_meta: {}, type_id: 3472328297305896040, name: "http" }, fields: [TransparentField { type_id: 3472328296231629683, name: "ssl", value: b"ssl", meta: FieldMetadata { name: "ssl", key: "ssl", type_id: 3472328296231629683, description: "SSL parameter." } }] }}, metadata: Metadata { items: {} }, extractors: Extractors { items: {} }, db: Some(InMemory { db_type: InMemory, items: Mutex { data: {}, poisoned: false, .. } }), config: BotnetConfig { version: V1, strategy: Strategy { entity: EntityCounter { enabled: true, counter: IpUa }, kanon: KAnon { k: K100, enabled: true }, cliff: CliffDetection { enabled: true, detector: V1 } }, keys: [Key { name: "http", fields: [TransparentField { name: "ssl", key: "ssl", description: "SSL parameter." }] }], database: Database { db_type: InMemory, uri: None } } }
2023-08-09T00:07:56.591955Z  INFO hello_world: 96: listening on 127.0.0.1:3000
```

Make request.

```bash
curl http://localhost:3000/
```