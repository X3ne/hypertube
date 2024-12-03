import { cn } from '@/lib/utils'
import React, { useState, useEffect, useRef, ReactNode } from 'react'

interface SlideshowProps {
  interval?: number
  children: ReactNode[]
  className?: string
  onChange?: (index: number) => void
}

const Slideshow: React.FC<SlideshowProps> = ({
  interval = 3000,
  className,
  children,
  onChange
}) => {
  const [currentIndex, setCurrentIndex] = useState<number>(0)
  const [progress, setProgress] = useState<number>(0)
  const timerRef = useRef<NodeJS.Timeout | null>(null)

  useEffect(() => {
    startTimer()
    if (onChange) onChange(currentIndex)
    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current)
      }
    }
  }, [currentIndex])

  useEffect(() => {
    if (progress >= 100) {
      nextSlide()
      resetProgress()
    }
  }, [progress])

  const startTimer = () => {
    resetProgress()
    timerRef.current = setInterval(() => {
      setProgress((prevProgress) => prevProgress + 1)
    }, interval / 100)
  }

  const resetProgress = () => {
    setProgress(0)
    if (timerRef.current) {
      clearInterval(timerRef.current)
    }
  }

  const nextSlide = () => {
    setCurrentIndex((prevIndex) => (prevIndex + 1) % children.length)
  }

  const handleTabClick = (index: number) => {
    resetProgress()
    setCurrentIndex(index)
  }

  return (
    <div
      className={cn('relative overflow-hidden', className)}
      onMouseEnter={resetProgress}
      onMouseLeave={startTimer}
    >
      <div className="relative w-full h-full">
        {children.map((child, index) => (
          <div
            key={index}
            className={`absolute inset-0 transition-opacity duration-1000 ease-in-out ${
              index === currentIndex ? 'opacity-100 z-20' : 'opacity-0 z-10'
            }`}
          >
            {child}
          </div>
        ))}
      </div>
      <div
        className={cn(
          'absolute bottom-20 left-0 z-20',
          'md:bottom-28 md:right-0 md:left-auto'
        )}
      >
        <div className="flex space-x-2 p-4">
          {children.map((_, index) => (
            <div
              key={index}
              onClick={() => handleTabClick(index)}
              className={'cursor-pointer w-12 py-1 overflow-hidden'}
            >
              {index !== currentIndex && (
                <div className="bg-foreground/40 w-full h-full rounded-full" />
              )}
              {index === currentIndex && (
                <div className="relative w-full h-1 bg-foreground/40 rounded-full">
                  <div
                    className="h-full bg-foreground transition-all rounded-full"
                    style={{ width: `${progress}%` }}
                  ></div>
                </div>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}

export default Slideshow
