# u32-matrix-extended

A web site and API designed to extend the functionality of [Matrix-Synapse](matrix.org).

# Layout

```
docs/
src/
  ui/ 
  api/
  build/
```

# Architecture

The application which guides the HTTP interactions is in `api/matrix-extended`.
*matrix-extended* is a modular monolith written in Rust.
It exposes various endpoints, some of which are designed to interact with the UI, others are designed to interact with applications.

## Endpoints

**TODO**:
 - [ ] Register | `api/register`
 - [ ] Login | `api/login`
 - [ ] Account | `api/account`
 - [ ] Sub-Account | `api/sub-account`
 - [ ] Send Message | `api/message`
