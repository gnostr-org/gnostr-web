# gnostr-hyper

An example of a minimal web service in Rust using hyper

Run with `make dev`

URLs to call:

```bash
curl http://localhost:8081/test

curl http://localhost:8081/params/1234

curl -X POST http://localhost:8081/send -d '{"name": "chip", "active": true }'
```
