import React from 'react'

interface CardContentProps {
  children: React.ReactNode
}

const CardContent: React.FC<CardContentProps> = ({ children }) => {
  return (
    <div className='bg-white min-h-[716px] w-full max-w-[900px] space-y-10 rounded-[24px] border-2 border-slate-600/20 p-12 shadow-2xl'>
      {children}
    </div>
  )
}

export default CardContent
