import { TV } from '@/api'
import { createContext, ReactNode, useEffect, useState } from 'react'

const getOpening = async (show: TV) => {
  if (!show || show.genres?.find((a) => a.name === 'Animation') === undefined) {
    return null
  }

  const ogShowName =
    show.alternative_titles?.results.find((a) => a.iso_3166_1 === 'en-US')
      ?.title || show.name

  try {
    const res = await fetch(
      `https://api.animethemes.moe/search?page[limit]=1&fields[search]=anime&q=${ogShowName}&include[anime]=animethemes.animethemeentries.videos&filter[year]=${show.first_air_date.slice(
        0,
        4
      )}`
    )

    const data = await res.json()

    console.debug('Fetched anime opening:', data)

    if (data.search.anime.length === 0) return null

    const op =
      data.search.anime[0].animethemes[0].animethemeentries[0].videos[0].link

    console.debug('Found OP for ' + show.name + ': ' + op)

    return op
  } catch (err) {
    console.error('Failed to fetch anime opening:', err)
    return null
  }
}

const fadeIn = (audio: HTMLAudioElement, duration: number) => {
  let volume = 0
  audio.volume = volume
  const fadeInterval = setInterval(() => {
    if (volume < 1) {
      volume += 0.05
      audio.volume = Math.min(volume, 1)
    } else {
      clearInterval(fadeInterval)
    }
  }, duration / 20)
}

const fadeOut = (
  audio: HTMLAudioElement,
  duration: number,
  onComplete: () => void
) => {
  let volume = 1
  const fadeInterval = setInterval(() => {
    if (volume > 0) {
      volume -= 0.05
      audio.volume = Math.max(volume, 0)
    } else {
      clearInterval(fadeInterval)
      audio.pause()
      onComplete()
    }
  }, duration / 20)
}

interface ISongContext {
  changeShow: (show: TV | null) => void
}

export const SongContext = createContext<ISongContext | undefined>(undefined)

interface SongProviderProps {
  children: ReactNode
}

export const SongProvider: React.FC<SongProviderProps> = ({
  children
}: SongProviderProps) => {
  const [show, setShow] = useState<TV | null>(null)
  const [audio, setAudio] = useState<HTMLAudioElement | null>(null)

  const changeShow = (show: TV | null) => {
    setShow(show)
  }

  useEffect(() => {
    let currentAudio: HTMLAudioElement | null = null

    if (show) {
      getOpening(show).then((url) => {
        if (!url) {
          console.warn('No opening found for ' + show.name)
          return
        }

        if (audio) {
          fadeOut(audio, 1000, () => setAudio(null))
        }

        currentAudio = new Audio(url)

        currentAudio.play().then(() => {
          if (!currentAudio) return
          fadeIn(currentAudio, 1000)
        })

        setAudio(currentAudio)
      })
    }

    return () => {
      if (currentAudio) {
        fadeOut(currentAudio, 1000, () => setAudio(null))
      }
    }
  }, [show])

  return (
    <SongContext.Provider value={{ changeShow }}>
      {children}
    </SongContext.Provider>
  )
}
