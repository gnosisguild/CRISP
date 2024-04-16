import { ReactNode } from 'react'
import * as WasmInstance from 'libs/wasm/pkg/crisp_web'

export type VoteManagementContextType = {
  isLoading: boolean
  wasmInstance: WasmInstance.InitOutput | null
  encryptInstance: WasmInstance.Encrypt | null
  initWebAssembly: () => Promise<void>
  encryptVote: (voteId: bigint, publicKey: Uint8Array) => Promise<Uint8Array | undefined>
}

export type VoteManagementProviderProps = {
  children: ReactNode
}
