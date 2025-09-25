"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const rust_lib_1 = require("./rust-lib");
const cache = new rust_lib_1.MiniCache();
cache.add('token', 'token value');
console.log(cache.get('token'));
cache.set('token', 'KLCJBREKCMAJSFDVKREIVKLSDJBVJERV');
console.log(cache.get('token'));
console.log(cache.all());
