import Gradient from '@/components/app/gradient'
import { extractColors, RGB } from '@/utils/extractColor'
import { createContext, ReactNode, useState } from 'react'

interface IGradientContext {
  image: string | null
  colors: RGB[]
  changeImage: (image: string | null) => void
}

export const GradientContext = createContext<IGradientContext | undefined>(
  undefined
)

interface GradientProviderProps {
  children: ReactNode
}

export const GradientProvider: React.FC<GradientProviderProps> = ({
  children
}: GradientProviderProps) => {
  const [image, setImage] = useState<string | null>(null)
  const [colors, setColors] = useState<RGB[]>([])

  const changeImage = (image: string | null) => {
    setImage(image)

    if (image) {
      extractColors(image).then((colors) => {
        setColors(colors)
      })
    } else {
      setColors([])
    }
  }

  return (
    <GradientContext.Provider value={{ image, colors, changeImage }}>
      <Gradient />
      {children}
    </GradientContext.Provider>
  )
}
