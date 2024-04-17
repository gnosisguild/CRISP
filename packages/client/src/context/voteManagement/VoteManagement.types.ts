import { ReactNode } from 'react'
import * as WasmInstance from 'libs/wasm/pkg/crisp_web'
import { SocialAuth } from '@/model/twitter.model'

export type VoteManagementContextType = {
  isLoading: boolean
  wasmInstance: WasmInstance.InitOutput | null
  encryptInstance: WasmInstance.Encrypt | null
  user: SocialAuth | null
  setUser: (value: SocialAuth | null) => void
  initWebAssembly: () => Promise<void>
  encryptVote: (voteId: bigint, publicKey: Uint8Array) => Promise<Uint8Array | undefined>
  logout: () => void
}

export type VoteManagementProviderProps = {
  children: ReactNode
}
