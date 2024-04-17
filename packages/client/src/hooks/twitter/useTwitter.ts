import { useState } from 'react'
import axios from 'axios'
import { handleGenericError } from '@/utils/handle-generic-error'
import { Twitter, SocialAuth } from '@/model/twitter.model'
import useLocalStorage from '@/hooks/generic/useLocalStorage'
import { AUTH_MSG } from '@/pages/Register/Register'
import { useVoteManagementContext } from '@/context/voteManagement'

const TWITTER_API = import.meta.env.VITE_TWITTER_SERVERLESS_API

if (!TWITTER_API) handleGenericError('useTwitter', { name: 'TWITTER_API', message: 'Missing env VITE_TWITTER_SERVERLESS_API' })

export const useTwitter = () => {
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [_socialAuth, setSocialAuth] = useLocalStorage<SocialAuth | null>('socialAuth', null)
  const { setUser } = useVoteManagementContext()

  const extractUsernameFromUrl = (url: string): string | null => {
    const regex = /https:\/\/twitter\.com\/([^\/]+)\/status\/\d+/
    const match = url.match(regex)
    return match ? match[1] : null
  }

  const handleTwitterPostVerification = async (postUrl: string) => {
    try {
      setIsLoading(true)
      const username = extractUsernameFromUrl(postUrl)
      const result = await axios.post<Twitter>(`${TWITTER_API}/twitter-data`, { url: postUrl })

      if (result.data) {
        const descriptionLowerCase = result.data.description.toLowerCase()
        const authMsgLowerCase = AUTH_MSG.toLowerCase()
        if (descriptionLowerCase.includes(authMsgLowerCase)) {
          const user = {
            validationDate: new Date(),
            avatar: result.data.open_graph.images[0].url ?? '',
            username: username ?? '',
          }
          setUser(user)
          setSocialAuth(user)
        }
      }
    } catch (error) {
      handleGenericError('handlePostVerification', error as Error)
    } finally {
      setIsLoading(false)
    }
  }

  return {
    isLoading,
    handleTwitterPostVerification,
  }
}
