import { useEffect, useState } from 'react'
import { ZKPClient } from 'enclave-circuits/src/client'
import { Buffer } from 'buffer'

let cache: ZKPClient | undefined

function useCircuit(): { client?: ZKPClient } {
  const [client, setClient] = useState<ZKPClient>()

  useEffect(() => {
    if (!cache) {
      Promise.all([
        fetch(`${import.meta.env.PUBLIC_URL}/libs/wasm/circuits/vote_integrity/vote_integrity.wasm`).then((res) => res.arrayBuffer()),
        fetch(`${import.meta.env.PUBLIC_URL}/libs/wasm/circuits/vote_integrity/vote_integrity_0001.zkey`).then((res) => res.arrayBuffer()),
      ]).then(([wasm, zkey]) => {
        console.log('wasm', wasm)
        console.log('zkey', zkey)
        new ZKPClient().init(Buffer.from(wasm), Buffer.from(zkey)).then((_client) => {
          if (!cache) {
            cache = _client
          }
          setClient(_client)
        })
      })
    } else {
      setClient(cache)
    }
  }, [])

  return { client }
}

export default useCircuit
