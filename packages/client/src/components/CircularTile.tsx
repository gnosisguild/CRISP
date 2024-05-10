import { useMediaQuery } from '@/hooks/generic/useMediaQuery'

interface CircularTileProps {
  className?: string
  rotation?: number
}

const CircularTile = ({ className, rotation }: CircularTileProps) => {
  const isMediumOrLarger = useMediaQuery('(min-width: 768px)')
  const viewBox = isMediumOrLarger ? '0 0 256 256' : '-80 -80 350 350'
  return (
    <svg
      className={`fill-slate-100 ${className}`}
      style={{ transform: `rotate(${rotation}deg)` }}
      viewBox={viewBox}
      xmlns='http://www.w3.org/2000/svg'
    >
      <path
        fillRule='evenodd'
        clipRule='evenodd'
        d='M85.6463 -8.912e-06C85.6463 47.1283 47.4413 85.3333 0.312983 85.3333L0.312988 256C141.698 256 256.313 141.385 256.313 -1.43382e-05L85.6463 -8.912e-06Z'
      />
    </svg>
  )
}

export default CircularTile
