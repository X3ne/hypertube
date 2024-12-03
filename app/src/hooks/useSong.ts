import { useContext } from 'react'
import { SongContext } from '@/providers/songProvider'

export const useSong = () => {
  const context = useContext(SongContext)

  if (!context) {
    throw new Error('useSong must be used within a SongProvider')
  }

  return context
}
