import Player from '@/components/app/player'
import { createContext, ReactNode, useState } from 'react'

export interface IMedia {
  name: string
  episode: string
  episode_name: string
  season: string
  cover: string
  stream_url: string
}

interface IPlaybackContext {
  media: IMedia | null
  changeMedia: (media: IMedia | null) => void
  stopPlayback: () => void
}

export const PlaybackContext = createContext<IPlaybackContext | undefined>(
  undefined
)

interface PlaybackProviderProps {
  children: ReactNode
}

export const PlaybackProvider: React.FC<PlaybackProviderProps> = ({
  children
}: PlaybackProviderProps) => {
  const [media, setMedia] = useState<IMedia | null>(null)

  const changeMedia = (new_media: IMedia | null) => {
    if (new_media === media) return

    setMedia(new_media)
  }

  const stopPlayback = () => {
    setMedia(null)
  }

  return (
    <PlaybackContext.Provider value={{ media, changeMedia, stopPlayback }}>
      <Player />
      {children}
    </PlaybackContext.Provider>
  )
}
