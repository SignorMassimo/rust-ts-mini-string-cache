import { MiniCache } from './rust-lib'

const cache = new MiniCache()

cache.add('token', 'token value')
console.log(cache.get('token'))
cache.set('token', 'KLCJBREKCMAJSFDVKREIVKLSDJBVJERV')
console.log(cache.get('token'))
console.log(cache.all())
