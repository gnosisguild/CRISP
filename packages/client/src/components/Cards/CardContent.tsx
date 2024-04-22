import React from 'react'

interface CardContentProps {
  children: React.ReactNode
}

const CardContent: React.FC<CardContentProps> = ({ children }) => {
  return (
    <div className='bg-white w-full max-w-screen-md space-y-10 rounded-2xl border-2 border-slate-600/20 p-8 md:p-12 shadow-2xl'>
      {children}
    </div>
  )
}

export default CardContent
