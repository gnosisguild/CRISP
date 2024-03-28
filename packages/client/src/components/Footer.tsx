import React from 'react'

const Footer: React.FC = () => {
  return (
    <footer className="w-full max-w-[900px] mx-auto border-t-2  border-twilight-blue-200 py-6 flex justify-between items-center h-16">
      <p className='font-bold text-sm'>CRISP Whitepaper</p>
      <p className='text-sm'>Built by {" "}<span className='font-bold'> Gnosis Guild</span></p>
    </footer>
  )
}

export default Footer