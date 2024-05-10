import { useEffect, useState } from 'react'
import CircularTile from '@/components/CircularTile'
import { useMediaQuery } from '@/hooks/generic/useMediaQuery'

const LoadingAnimation = ({ className, isLoading }: { className?: string; isLoading: boolean }) => {
  const [rotations, setRotations] = useState([0, 0, 0, 0])

  // Determine if the screen width is medium or larger
  const isMobile = useMediaQuery('(max-width: 767px)')
  const isTablet = useMediaQuery(
    '(min-width: 768px) and (max-device-width: 1180px), (min-device-width: 768px) and (max-device-width: 1180px)',
  )
  const isDesktop = useMediaQuery('(min-width: 1181px)')

  const getRandRotation = () => {
    const rand_index = Math.floor(Math.random() * 4)
    const rotation = [0, 90, 180, 270][rand_index]
    return rotation
  }

  useEffect(() => {
    const interval = setInterval(() => {
      if (isLoading) {
        setRotations([getRandRotation(), getRandRotation(), getRandRotation(), getRandRotation()])
      }
    }, 500)

    if (!isLoading) {
      clearInterval(interval)
    }

    return () => clearInterval(interval)
  }, [rotations, isLoading])

  // Adjust size based on screen width
  const sizeClasses = isDesktop ? 'w-10 h-10' : isTablet ? 'w-5 h-5' : 'w-7 h-7'
  const gapClasses = isDesktop ? 'gap-1' : isTablet ? 'gap-5 mr-4 mb-4' : 'gap-4 mr-4'
  return (
    <div className={`flex h-full items-center justify-center ${className}`}>
      <div className={`grid grid-cols-2 ${gapClasses} ${sizeClasses}`}>
        {rotations.map((rotation, i) => {
          return <CircularTile key={i} className='!fill-slate-600 duration-500 ease-in-out' rotation={rotation} />
        })}
      </div>
    </div>
  )
}

export default LoadingAnimation
