import { useState } from 'react'
import * as WasmInstance from 'libs/wasm/pkg/crisp_web'
import { handleGenericError } from '@/utils/handle-generic-error'

export const useWebAssemblyHook = () => {
  const [wasmInstance, setWasmInstance] = useState<WasmInstance.InitOutput | null>(null)
  const [encryptInstance, setEncryptInstance] = useState<WasmInstance.Encrypt | null>(null)
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const initWebAssembly = async () => {
    try {
      const wasmModule = await WasmInstance.default()
      const newEncryptInstance = new WasmInstance.Encrypt()
      setWasmInstance(wasmModule)
      setEncryptInstance(newEncryptInstance)
    } catch (err) {
      handleGenericError('initWebAssembly', err as Error)
    } finally {
      setIsLoading(false)
    }
  }

  const encryptVote = async (voteId: bigint, publicKey: Uint8Array): Promise<Uint8Array | undefined> => {
    if (!encryptInstance) {
      console.error('WebAssembly module not initialized')
      return
    }
    try {
      setIsLoading(true)
      return encryptInstance.encrypt_vote(voteId, publicKey)
    } catch (err) {
      console.log('err', err)
      handleGenericError('encryptVote', err as Error)
    }
  }

  return {
    isLoading,
    wasmInstance,
    encryptInstance,
    initWebAssembly,
    encryptVote,
  }
}
