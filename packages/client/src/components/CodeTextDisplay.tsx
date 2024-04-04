import React, { useState } from 'react'
import { PrismLight as SyntaxHighlighter } from 'react-syntax-highlighter'
import { oneLight } from 'react-syntax-highlighter/dist/esm/styles/prism'
import PaperIcon from '../assets/icons/paper.svg'
import PaperPurpleIcon from '../assets/icons/paperPurple.svg'
import FingerprintIcon from '../assets/icons/fingerprint.svg'
import FingerprintWhiteIcon from '../assets/icons/fingerprintWhite.svg'

interface CodeTextDisplayProps {}

const selectedClass = 'border-twilight-blue-800 flex space-x-2 rounded-lg border-2 bg-white-900 px-4 py-2'
const unSelectedClass = 'flex space-x-2 rounded-lg border-2 border-twilight-blue-200 bg-[#B7BBC1] px-4 py-2'

const CodeTextDisplay: React.FC<CodeTextDisplayProps> = () => {
  const text = `import React from 'react'

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
  `

  const [isCipher, setIsCipher] = useState<boolean>(true)

  return (
    <div className='rounded-lg shadow'>
      <div className='flex space-x-2 rounded-t-lg border-x-2 border-t-2 border-twilight-blue-200  bg-twilight-blue-100 px-4 py-2'>
        <button className={isCipher ? unSelectedClass : selectedClass} onClick={() => setIsCipher(false)}>
          <img src={isCipher ? PaperIcon : PaperPurpleIcon} />
          <p className={`${isCipher ? 'text-white-800' : 'text-purple-900'} text-base font-semibold`}>Plain Text</p>
        </button>
        <button className={isCipher ? selectedClass : unSelectedClass} onClick={() => setIsCipher(true)}>
          <img src={isCipher ? FingerprintIcon : FingerprintWhiteIcon} />
          <p className={`${!isCipher ? 'text-white-800' : 'text-purple-900'} text-base font-semibold`}>Cypher Text</p>
        </button>
      </div>
      <div className=' rounded-b-lg border-2 border-twilight-blue-200 p-5'>
        {text ? (
          isCipher ? (
            <SyntaxHighlighter language='javascript' style={oneLight}>
              {text}
            </SyntaxHighlighter>
          ) : (
            text
          )
        ) : (
          'Loading...'
        )}
      </div>
    </div>
  )
}

export default CodeTextDisplay
