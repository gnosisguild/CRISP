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
        ${isDetails ? ' p-4' : 'h-auto min-h-[288px] p-20 sm:w-full md:w-[434px]'}
        bg-white rounded-[24px] text-black
        ${!isDetails && 'shadow-custom-1'}
        transform 
        border-2 transition-all duration-300 ease-in-out 
        ${isClicked ? 'scale-105 border-green-light' : ''}
        ${isClicked ? 'border-green-light' : 'border-twilight-blue-200'}
        ${isClicked ? 'bg-white-900' : 'bg-white-500'}
        ${!isDetails && 'hover:shadow-lg'}
        flex items-center justify-center
      `}
      onClick={handleClick}
    >
      {children}
    </div>
  )
}

export default Card
