// src/hooks/semaphore/useSemaphoreIdentity.ts
import { Identity } from "@semaphore-protocol/identity"
import { handleGenericError } from '@/utils/handle-generic-error'
import { SemaphoreRegistrationRequest, SemaphoreRegistrationResponse } from '@/model/vote.model'
import { useApi } from '../generic/useFetchApi'

const ENCLAVE_API = import.meta.env.VITE_ENCLAVE_API

if (!ENCLAVE_API) handleGenericError('useSemaphoreIdentity', { name: 'ENCLAVE_API', message: 'Missing env VITE_ENCLAVE_API' })

const SemaphoreEndpoints = {
    RegisterIdentity: `${ENCLAVE_API}/rounds/register`,
} as const

export const useSemaphoreIdentity = () => {
    const { RegisterIdentity } = SemaphoreEndpoints
    const { fetchData, isLoading } = useApi()

    const createIdentityFromCommitteeKey = (publicKey: Uint8Array | number[]): Identity => {
        try {
            // Convert the committee public key to a hex string
            const keyString = Array.from(publicKey)
                .map(byte => byte.toString(16).padStart(2, '0'))
                .join('')

            // Create a deterministic Semaphore identity
            return new Identity(keyString)
        } catch (error) {
            handleGenericError('createIdentityFromCommitteeKey', error as Error)
            throw error
        }
    }

    const registerWithSemaphoreGroup = (request: SemaphoreRegistrationRequest) =>
        fetchData<SemaphoreRegistrationResponse, SemaphoreRegistrationRequest>(
            RegisterIdentity,
            'post',
            request
        )

    return {
        isLoading,
        createIdentityFromCommitteeKey,
        registerWithSemaphoreGroup,
    }
}