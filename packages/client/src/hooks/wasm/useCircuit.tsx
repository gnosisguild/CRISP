import { useState } from 'react'
import { Proof, ZKPClient } from 'enclave-circuits/src/client'
import { Buffer } from 'buffer'
import { handleGenericError } from '@/utils/handle-generic-error'
import { useNotificationAlertContext } from '@/context/NotificationAlert'

let cache: ZKPClient | undefined

export const useCircuitHook = () => {
  const { showToast } = useNotificationAlertContext()
  const [zpkClient, setZPKClient] = useState<ZKPClient | null>(null)
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const initCircuits = async () => {
    if (!cache) {
      try {
        setIsLoading(true)
        const [wasmBuffer, zkeyBuffer] = await Promise.all([
          fetch('libs/wasm/circuits/vote_integrity/vote_integrity.wasm').then((res) => res.arrayBuffer()),
          fetch('libs/wasm/circuits/vote_integrity/vote_integrity_0001.zkey').then((res) => res.arrayBuffer()),
        ])

        const newClient = new ZKPClient()
        await newClient.init(Buffer.from(wasmBuffer), Buffer.from(zkeyBuffer))

        cache = newClient
        setZPKClient(newClient)
      } catch (err) {
        showToast({
          type: 'danger',
          message: 'Failed to initialize ZKPClient',
        })
        handleGenericError('initCircuit', err as Error)
      } finally {
        setIsLoading(false)
      }
    } else {
      setZPKClient(cache)
    }
  }

  const proveVote = async (vote: number): Promise<Proof | undefined> => {
    if (!zpkClient) {
      console.error('ZKPClient not initialized')
      return
    }
    try {
      setIsLoading(true)
      return await zpkClient.prove({ vote })
    } catch (err) {
      showToast({
        type: 'danger',
        message: 'Failed to generate proof',
      })
      handleGenericError('proveVote', err as Error)
    } finally {
      setIsLoading(false)
    }
  }

  return {
    isLoading,
    zpkClient,
    initCircuits,
    proveVote,
  }
}

export default useCircuitHook
