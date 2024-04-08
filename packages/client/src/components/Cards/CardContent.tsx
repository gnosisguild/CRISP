import React from 'react'

interface CardContentProps {
  children: React.ReactNode
}

const CardContent: React.FC<CardContentProps> = ({ children }) => {
  return (
    <div className='min-h-[716px] w-full max-w-[900px] space-y-10 rounded-[24px] border-2 border-twilight-blue-200 bg-white-900 p-12 shadow-modal'>
      {children}
    </div>
  )
}

export default CardContent
