<p align="center">
  <img src="https://raw.githubusercontent.com/sdslabs/VortexDB/refactor/assets/logo.svg" alt="VortexDB Logo" height="100">
</p>

<!-- Make sure to change the logo from light to dark whenever packaging for PyPI. 
    Revert to light logo afterwards -->

# VortexDB Python Client

This is the official Python client for **VortexDB**, a vector database written in Rust 🦀 exposed via a gRPC API.

The client provides a thin, typed, Pythonic wrapper over the VortexDB gRPC interface, handling:
- connection setup
- authentication
- request/response mapping
- error translation
- resource cleanup


The client is designed to be minimal, explicit, and easy to extend.

---

## Requirements

- Python `3.10+`
- A running VortexDB gRPC server

---

## Help

For setup instructions and usage see:  
[VortexDB Usage Guide](https://github.com/sdslabs/VortexDB/tree/refactor/client/python/USAGE.md)

---
<br>
<p align="center">
Made with ❤️ by SDSLabs
</p>