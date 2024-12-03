import { useContext } from 'react'
import { GradientContext } from '@/providers/gradientProvider'

export const useGradient = () => {
  const context = useContext(GradientContext)

  if (!context) {
    throw new Error('useGradient must be used within a GradientProvider')
  }

  return context
}
