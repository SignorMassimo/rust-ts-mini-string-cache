"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const index_1 = require("./index");
const cache = new index_1.MiniCache();
cache.add('token', 'token value');
console.log(cache.get('token'));
cache.set('token', 'KLCJBREKCMAJSFDVKREIVKLSDJBVJERV');
console.log(cache.get('token'));
console.log(cache.all());
