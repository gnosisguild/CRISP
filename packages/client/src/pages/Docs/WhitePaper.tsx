import { ArrowUp } from '@phosphor-icons/react'
import { useEffect, useMemo, useState } from 'react'
import Markdown from 'react-markdown'
import { markdown } from '@/utils/whitepaper'
import { Components } from 'react-markdown'
import CircularTiles from '../../components/CircularTiles'

const WhitePaper = () => {
  const [scrolledBelowContents, setScrolledBelow] = useState(false)

  const scrollToElement = (id: string) => {
    const element = document.getElementById(id)
    if (element) {
      element.scrollIntoView({ behavior: 'smooth', block: 'start' })
    }
  }

  const components: Components = useMemo(
    () => ({
      h1: ({ node, ...props }) => (
        <h1 id={props.children ? props.children.toString().toLowerCase().replace(/ /g, '-') : ''}>{props.children}</h1>
      ),
      h2: ({ node, ...props }) => (
        <h2 id={props.children ? props.children.toString().toLowerCase().replace(/ /g, '-') : ''}>{props.children}</h2>
      ),
      h3: ({ node, ...props }) => (
        <h3 id={props.children ? props.children.toString().toLowerCase().replace(/ /g, '-') : ''}>{props.children}</h3>
      ),
      h4: ({ node, ...props }) => (
        <h4 id={props.children ? props.children.toString().toLowerCase().replace(/ /g, '-') : ''}>{props.children}</h4>
      ),
      h5: ({ node, ...props }) => (
        <h5 id={props.children ? props.children.toString().toLowerCase().replace(/ /g, '-') : ''}>{props.children}</h5>
      ),
      a: ({ node, ...props }) => (
        <a
          href={props.href}
          onClick={(e) => {
            e.preventDefault()
            const id = props.href?.split('#').pop() as string
            scrollToElement(id)
          }}
          className='cursor-pointer no-underline hover:underline'
        >
          {props.children}
        </a>
      ),
      img: ({ node, ...props }) => (
        <img src={props.src} className='h-auto w-full rounded-2xl border-2 border-slate-300'>
          {props.children}
        </img>
      ),
    }),
    [],
  )

  useEffect(() => {
    const footerHeight = document.querySelector('footer')?.clientHeight || 76
    const handleScroll = () => {
      const graphic = document.querySelector('.fixed-graphic') as HTMLElement
      const contentsLink = document.querySelector('#contents-link') as HTMLElement
      const fromBottom = window.innerHeight + window.scrollY - document.body.offsetHeight + footerHeight
      if (contentsLink) {
        if (fromBottom > 0 && graphic?.style && contentsLink?.style) {
          graphic.style.bottom = `${fromBottom}px`
          contentsLink.style.bottom = `calc(${fromBottom}px + 1.5rem)`
        } else {
          graphic.style.bottom = '0.25rem'
          contentsLink.style.bottom = '1.5rem'
        }
      }
      const contentsSection = document.querySelector('#contents') as HTMLElement
      setScrolledBelow(!!(contentsSection && window.scrollY > contentsSection?.offsetTop + contentsSection?.clientHeight))
    }

    window.addEventListener('scroll', handleScroll)
    return () => window.removeEventListener('scroll', handleScroll)
  }, [])

  return (
    <div className='relative min-h-screen w-screen px-6 py-28'>
      <div className='fixed-graphic fixed bottom-0 right-0 grid w-[40vh] grid-cols-2 gap-2 max-md:opacity-50'>
        <CircularTiles count={4} />
      </div>
      <div className='relative mx-auto w-full max-w-screen-md'>
        <Markdown components={components} className='prose prose-p:leading-loose prose-headings:mt-8 prose-h4:text-base prose-h5:text-sm'>
          {markdown}
        </Markdown>
        {scrolledBelowContents && (
          <a
            id='contents-link'
            href='#contents'
            onClick={(e) => {
              e.preventDefault()
              scrollToElement('contents')
            }}
            className='fixed bottom-6 right-6 z-10 flex cursor-pointer flex-col items-center space-y-1 rounded-lg bg-white/50 p-2 shadow-lg backdrop-blur-xl hover:bg-white'
          >
            <ArrowUp size={32} weight='bold' />
            <div className='text-center text-sm'>Go to contents</div>
          </a>
        )}
      </div>
    </div>
  )
}

export default WhitePaper
