import { usePlayback } from '@/hooks/usePlayback'
import { cn } from '@/lib/utils'
import { formatDuration } from '@/utils'
import React, { useEffect } from 'react'
import {
  LuChevronDown,
  LuChevronUp,
  LuLoaderCircle,
  LuMaximize2,
  LuPause,
  LuPlay,
  LuSquare
} from 'react-icons/lu'
import ReactPlayer from 'react-player'
import { OnProgressProps } from 'react-player/base'
import * as portals from 'react-reverse-portal'

const Player = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => {
  const [fullscreen, setFullscreen] = React.useState<
    'thumb' | 'fullscreen' | 'page-fullscreen'
  >('thumb')
  const [progress, setProgress] = React.useState<OnProgressProps>({
    played: 0,
    playedSeconds: 0,
    loaded: 0,
    loadedSeconds: 0
  })
  const [duration, setDuration] = React.useState(0)
  const [playing, setPlaying] = React.useState(false)
  const [showControls, setShowControls] = React.useState(true)
  const [isLoading, setIsLoading] = React.useState(false)
  const [error, setError] = React.useState<string | null>(null)

  const { media, stopPlayback } = usePlayback()

  const portalNode = React.useMemo(() => portals.createHtmlPortalNode(), [])

  const player = React.useRef<ReactPlayer>(null)

  const isFullScreen =
    fullscreen === 'fullscreen' || fullscreen === 'page-fullscreen'

  useEffect(() => {
    document.addEventListener(
      'mousemove',
      () => {
        if (isFullScreen) {
          setShowControls(true)
        }
      },
      { passive: true }
    )

    return () => {
      document.removeEventListener('mousemove', () => setShowControls(true))
    }
  }, [isFullScreen])

  useEffect(() => {
    if (!isFullScreen) {
      setShowControls(true)
    }

    let timeout: NodeJS.Timeout
    if (showControls) {
      timeout = setTimeout(() => {
        setShowControls(false)
      }, 3000)
    }

    return () => {
      clearTimeout(timeout)
    }
  }, [showControls, isFullScreen])

  if (media === null) {
    return
  }

  const handleSeekTo = (e: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    const rect = e.currentTarget.getBoundingClientRect()
    const x = e.clientX - rect.left
    const percentage = x / rect.width
    player.current?.seekTo(percentage, 'fraction')
  }

  const handleVideoFullScreen = () => {
    const root = document.documentElement

    if (root.requestFullscreen) {
      root.requestFullscreen()
    }
  }

  return (
    <div
      className={cn(
        className,
        isFullScreen
          ? 'fixed top-0 left-0 w-screen h-screen z-40 bg-black'
          : 'relative'
      )}
      ref={ref}
      {...props}
    >
      <portals.InPortal node={portalNode}>
        <ReactPlayer
          ref={player}
          url={media?.stream_url}
          controls={false}
          playing={playing}
          width="100%"
          height="100%"
          onProgress={(state) => setProgress(state)}
          onDuration={(duration) => setDuration(duration)}
          onBuffer={() => setIsLoading(true)}
          onBufferEnd={() => setIsLoading(false)}
          onError={(err) => setError(err)}
        />
        {isLoading && (
          <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black bg-opacity-50 z-20">
            <LuLoaderCircle className="w-10 h-10 animate-spin" />
          </div>
        )}
        {error && (
          <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black bg-opacity-50 z-20">
            <p className="text-white">{error}</p>
          </div>
        )}
      </portals.InPortal>

      {isFullScreen && (
        <div className="fixed flex items-center justify-center top-0 left-0 w-screen h-screen">
          <portals.OutPortal node={portalNode} />
          <div
            className={cn(
              'fixed flex flex-row items-center justify-between left-0 top-0 w-full px-6 py-4 bg-accent/60 backdrop-blur-md z-30 transition-transform duration-500 ease-in-out',
              {
                '-translate-y-14': !showControls
              }
            )}
          >
            <button onClick={() => setFullscreen('thumb')}>
              <LuChevronDown className="w-5 h-5" />
            </button>
            <button onClick={handleVideoFullScreen}>
              <LuMaximize2 className="w-5 h-5" />
            </button>
          </div>
        </div>
      )}
      <div
        className={cn(
          'fixed bottom-0 left-0 h-24 w-full bg-accent/60 backdrop-blur-md z-30 transition-transform duration-500 ease-in-out',
          {
            'translate-y-24': !showControls
          }
        )}
      >
        <div
          className={cn('relative w-full h-1 bg-accent cursor-pointer', {
            relative: !isFullScreen
          })}
          onClick={handleSeekTo}
        >
          <span
            className="absolute top-0 left-0 h-full bg-white/50 transition-width duration-500 ease-in-out"
            style={{ width: `${progress.loaded * 100}%` }}
          ></span>
          <span
            className="absolute top-0 left-0 h-full bg-foreground transition-width duration-500 ease-in-out"
            style={{ width: `${progress.played * 100}%` }}
          ></span>
        </div>
        <div className="flex flex-row justify-between items-center h-full w-full py-2 px-4">
          <div
            className={cn('flex flex-row w-5/12', { 'gap-4': !isFullScreen })}
          >
            <div>
              {!isFullScreen && (
                <div
                  className="relative group w-36 aspect-video bg-black rounded-sm overflow-hidden cursor-pointer"
                  onClick={() => setFullscreen('page-fullscreen')}
                >
                  <portals.OutPortal node={portalNode} />
                  <div className="absolute top-0 left-0 flex items-center justify-center w-full h-full bg-accent/60 opacity-0 transition-opacity duration-300 group-hover:opacity-100">
                    <LuChevronUp className="w-5 h-5" />
                  </div>
                </div>
              )}
            </div>
            <div className="grow py-2 text-sm truncate text-ellipsis space-y-[1px]">
              <p className="text-ellipsis w-full">{media?.name}</p>
              <div className="flex flex-row gap-2 text-foreground/70">
                <p className="font-mono">
                  S{media?.season}-E{media?.episode}
                </p>
                <p className="text-ellipsis"> - {media?.episode_name}</p>
              </div>
              <div className="flex flex-row gap-1 font-mono text-foreground/70">
                <p>{formatDuration(progress.playedSeconds)}</p>
                <p className="text-[12px]">/</p>
                <p>{duration !== 0 ? formatDuration(duration) : '--:--'}</p>
              </div>
            </div>
          </div>
          <div className="flex items-center gap-4">
            <button
              className="flex items-center justify-center p-2 bg-accent rounded-full"
              onClick={() => setPlaying(!playing)}
            >
              {playing ? (
                <LuPause className="fill-foreground stroke-foreground" />
              ) : (
                <LuPlay className="fill-foreground stroke-foreground" />
              )}
            </button>
            <button onClick={stopPlayback}>
              <LuSquare className="w-3.5 h-3.5 fill-muted-foreground stroke-muted-foreground" />
            </button>
          </div>
          <div className="w-5/12"></div>
        </div>
      </div>
    </div>
  )
})
Player.displayName = 'Player'

export default Player
