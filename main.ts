import { MiniCache } from './index'

const cache = new MiniCache()
//! add - get
cache.add('token', 'user-token')
console.log(cache.get('token'))
//! delete - has
// console.log(cache.has('token'))
// cache.delete('token')
// console.log(cache.has('token'))
//! filter - all
console.log(cache.all())
cache.add('env', '.env')
console.log(cache.filter(['token']))
//! set
cache.set('token', 'token user')
console.log(cache.get('token'))
