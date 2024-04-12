import React, { useEffect, useState } from 'react'

interface CardProps {
  children: React.ReactNode
  isDetails?: boolean
  checked?: boolean
  onChecked?: (clicked: boolean) => void
}

const Card: React.FC<CardProps> = ({ children, isDetails, checked, onChecked }) => {
  const [isClicked, setIsClicked] = useState<boolean>(checked ?? false)

  useEffect(() => {
    setIsClicked(checked ?? false)
  }, [checked])

  const handleClick = () => {
    if (isDetails) return
    if (onChecked) onChecked(!isClicked)
    setIsClicked(!isClicked)
  }

  return (
    <div
      className={`
        h-full
        cursor-pointer
        ${isDetails ? ' p-4' : 'h-auto min-h-[288px] p-20'}
        bg-white rounded-[24px] text-black
        ${!isDetails && 'shadow-md'}
        transform 
        border-2 transition-all duration-300 ease-in-out 
        ${isClicked ? 'scale-105 border-lime-400' : ''}
        ${isClicked ? 'border-lime-400' : 'border-slate-600/20'}
        ${isClicked ? 'bg-white' : 'bg-white/50'}
        ${!isDetails && 'hover:bg-white hover:shadow-lg'}
        flex w-full items-center justify-center
      `}
      onClick={handleClick}
    >
      {children}
    </div>
  )
}

export default Card
