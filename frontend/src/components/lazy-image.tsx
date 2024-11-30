import React, { useEffect, useState, useRef } from 'react'
import { cn } from '@/lib/utils'

export interface ShowHoverProps extends React.HTMLAttributes<HTMLImageElement> {
  src: string
  placeholder?: string
  alt?: string
}

const LazyImage = React.forwardRef<HTMLImageElement, ShowHoverProps>(
  ({ className, src, alt, placeholder, ...props }, ref) => {
    const [isVisible, setIsVisible] = useState(false)
    const imageRef = useRef<HTMLImageElement | null>(null)

    const setRef = (node: HTMLImageElement | null) => {
      imageRef.current = node
      if (typeof ref === 'function') {
        ref(node)
      } else if (ref) {
        ;(ref as React.MutableRefObject<HTMLImageElement | null>).current = node
      }
    }

    useEffect(() => {
      const observer = new IntersectionObserver(
        ([entry]) => {
          if (entry.isIntersecting) {
            setIsVisible(true)
            if (imageRef.current) {
              observer.unobserve(imageRef.current)
            }
          }
        },
        {
          root: null,
          rootMargin: '0px',
          threshold: 0.1
        }
      )

      const currentRef = imageRef.current
      if (currentRef) {
        observer.observe(currentRef)
      }

      return () => {
        if (currentRef) {
          observer.unobserve(currentRef)
        }
      }
    }, [src])

    return (
      <div className={cn('relative overflow-hidden', className)}>
        <img
          ref={setRef}
          src={isVisible ? src : placeholder}
          alt={alt}
          className={cn(!isVisible && 'blur-sm', 'w-full h-full object-cover')}
          {...props}
        />
      </div>
    )
  }
)

LazyImage.displayName = 'LazyImage'
export default LazyImage
