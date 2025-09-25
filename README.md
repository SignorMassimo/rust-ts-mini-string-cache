# MiniCache

A simple **in-memory cache** implemented in **Rust** and exposed to **Node.js** using [NAPI-RS](https://napi.rs/). This project was built as a **practice exercise** to learn Rust + TypeScript integration, not as a production-ready package.

## Features

* Add key/value pairs to the cache
* Update existing values
* Retrieve values by key
* Get all stored values

## Usage

```ts
import { MiniCache } from "./rust-lib";

const cache = new MiniCache();

// Add
cache.add("token", "token value");
console.log(cache.get("token")); // "token value"

// Update
cache.set("token", "NEW_TOKEN_VALUE");
console.log(cache.get("token")); // "NEW_TOKEN_VALUE"

// Get all
console.log(cache.all()); // [{ key: "token", value: "NEW_TOKEN_VALUE" }]
```

## Project Goal

This project was created as a **learning exercise**:

* To get comfortable with **Rust** and its ecosystem
* To integrate Rust with **TypeScript** via NAPI
* To explore how such bindings can be used in **real projects** later

👉 It is **not intended** to be published or used as a real npm package.

---

⚡ Built with ❤️ using Rust + TypeScript
