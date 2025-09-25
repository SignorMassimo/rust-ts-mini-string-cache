import { MiniCache } from './index'

const cache = new MiniCache()

cache.add('token', 'token value')
console.log(cache.get('token'))
cache.set('token', 'KLCJBREKCMAJSFDVKREIVKLSDJBVJERV')
console.log(cache.get('token'))
console.log(cache.all())
