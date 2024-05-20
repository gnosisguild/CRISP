import { handleGenericError } from '@/utils/handle-generic-error'
import { Twitter, SocialAuth } from '@/model/twitter.model'
import useLocalStorage from '@/hooks/generic/useLocalStorage'
import { useVoteManagementContext } from '@/context/voteManagement'
import { useApi } from '@/hooks/generic/useFetchApi'

const TWITTER_API = import.meta.env.VITE_TWITTER_SERVERLESS_API

if (!TWITTER_API) handleGenericError('useTwitter', { name: 'TWITTER_API', message: 'Missing env VITE_TWITTER_SERVERLESS_API' })

// Regex to match the expected Twitter post description
const MSG_REGEX = /🤫 I am authenticating with my Twitter account to cast my first encrypted vote with CRISP!\n\n#FHE #ZKP #CRISP/i

export const useTwitter = () => {
  const url = `${TWITTER_API}/twitter-data`
  const { fetchData, isLoading } = useApi()
  const [_socialAuth, setSocialAuth] = useLocalStorage<SocialAuth | null>('socialAuth', null)
  const { setUser, getToken } = useVoteManagementContext()

  // Function to extract username from Twitter URL
  const extractUsernameFromUrl = (url: string): string | null => {
    const regex = /https:\/\/[^\/]+\/([^\/]+)\/status\/\d+/
    const match = url.match(regex)
    return match ? match[1] : null
  }

  // Function to extract post ID from Twitter URL
  const extractPostId = (url: string): string | null => {
    const regex = /\/status\/(\d+)$/
    const match = url.match(regex)
    return match ? match[1] : null
  }

  // Function to handle Twitter post verification
  const handleTwitterPostVerification = async (postUrl: string) => {
    const username = extractUsernameFromUrl(postUrl)
    const result = await verifyPost(postUrl)

    if (result) {
      // Convert the description to lowercase for case-insensitive matching
      const descriptionLowerCase = result.description.toLowerCase()

      // Check if the description matches the regex
      if (MSG_REGEX.test(descriptionLowerCase)) {
        const postId = extractPostId(result.open_graph.url) ?? ''
        const token = await getToken(postId)

        if (token && ['Authorized', 'Already Authorized'].includes(token.response)) {
          const user = {
            validationDate: new Date(),
            avatar: result.open_graph.images[0].url ?? '',
            username: username ?? '',
            postId,
            token: token.jwt_token,
          }
          setUser(user)
          setSocialAuth(user)
        }
      }
    }
  }

  // API call to verify the Twitter post
  const verifyPost = (postUrl: string) => fetchData<Twitter, { url: string }>(url, 'post', { url: postUrl })

  return {
    isLoading,
    handleTwitterPostVerification,
  }
}
