import { useEffect, useState } from 'react'

export const useMediaQuery = (query: string): boolean => {
  const [matches, setMatches] = useState<boolean>(false)

  useEffect(() => {
    const media = window.matchMedia(query)
    const listener = (event: MediaQueryListEvent) => {
      setMatches(event.matches)
    }

    media.addEventListener('change', listener)

    if (media.matches !== matches) {
      setMatches(media.matches)
    }

    return () => {
      media.removeEventListener('change', listener)
    }
  }, [matches, query])

  return matches
}
