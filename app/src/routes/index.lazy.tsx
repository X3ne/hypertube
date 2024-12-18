import Slideshow from '@/components/app/slideshow'
import { useQuery } from '@tanstack/react-query'
import createDOMPurify from 'dompurify'
import { createLazyFileRoute, Link } from '@tanstack/react-router'
import { Button } from '@/components/ui/button'
import { LuArrowRight, LuPause, LuPlay } from 'react-icons/lu'
import { MovieCarousel, TvCarousel } from '@/components/app/carousel'
import { useEffect, useState } from 'react'
import { cn } from '@/lib/utils'
import { useGradient } from '@/hooks/useGradient'
import { useSong } from '@/hooks/useSong'
import api from '@/api'
import { getTmdbImageUrl } from '@/utils/tmdb'
import ReactPlayer from 'react-player'

export const Route = createLazyFileRoute('/')({
  component: Index
})

function Index() {
  const { changeImage } = useGradient()
  const { changeShow } = useSong()

  useEffect(() => {
    changeImage(null)
    changeShow(null)
  }, [changeImage, changeShow])

  const { data: tvTrending } = useQuery({
    queryKey: ['tv_trending'],
    queryFn: async () => {
      return (await api.getTvTrending()).data
    },
    refetchOnWindowFocus: false
  })
  const { data: movieTrending } = useQuery({
    queryKey: ['movie_trending'],
    queryFn: async () => {
      return (await api.getMovieTrending()).data
    },
    refetchOnWindowFocus: false
  })

  const DOMPurify = createDOMPurify(window)

  const [isVideoEnabled, setIsVideoEnabled] = useState(
    localStorage.getItem('video-enabled') === 'true'
  )

  const videosIds = tvTrending?.slice(0, 4).map((show) => {
    return (
      show.videos?.results.filter(
        (a) => a.site === 'YouTube' && a.type === 'Trailer'
      )[0]?.key || ''
    )
  })

  useEffect(() => {
    localStorage.setItem('video-enabled', String(isVideoEnabled))
  }, [isVideoEnabled])

  return (
    <>
      <div className="relative h-dvh">
        {tvTrending && (
          <>
            <Slideshow interval={10000} className="w-full h-[560px]">
              {tvTrending.slice(0, 4).map((show, i) => (
                <div
                  className="relative w-full h-full text-foreground"
                  key={show.id}
                >
                  {videosIds && videosIds[i] && isVideoEnabled ? (
                    <div className="relative w-full h-full">
                      <div className="relative pointer-events-none w-full aspect-video scale-[3] lg:scale-150 md:-translate-y-64 -z-10">
                        <ReactPlayer
                          url={`https://www.youtube.com/watch?v=${videosIds[i]}`}
                          width={'100%'}
                          height={'100%'}
                          playing={true}
                          muted={true}
                          controls={false}
                          loop={true}
                        />
                      </div>
                      <div className="absolute top-0 left-0 w-full h-full bg-white/0 z-10"></div>
                    </div>
                  ) : (
                    <img
                      src={getTmdbImageUrl(show.backdrop_path) || ''}
                      alt={show.name}
                      className="w-full h-full object-cover"
                    />
                  )}
                  <div className="bg-gradient-to-t from-background to-transparent absolute inset-0" />
                  <div className="absolute bottom-28 p-4 space-y-4 z-20">
                    <h1 className="font-bold text-3xl md:text-5xl">
                      {show.name}
                    </h1>
                    <p
                      className="w-2/3 md:w-2/5 line-clamp-3 text-sm"
                      dangerouslySetInnerHTML={{
                        __html: DOMPurify.sanitize(show.overview)
                      }}
                    />
                    <div className="flex flex-row gap-2">
                      <Button className="gap-1 px-8">
                        <LuPlay className="fill-black" />
                        Watch Pv
                      </Button>
                      <Link to={`/show/tv/${show.id}`}>
                        <Button variant="outline" className="gap-1 px-8">
                          More info
                          <LuArrowRight className="h-4 w-4" />
                        </Button>
                      </Link>
                      <Button
                        variant="outline"
                        className="group flex justify-center items-center h-10 w-10 p-0"
                        onClick={() => setIsVideoEnabled(!isVideoEnabled)}
                      >
                        <LuPlay
                          className={cn(
                            'absolute fill-foreground stroke-foreground opacity-0 transition-opacity',
                            'group-hover:fill-background group-hover:stroke-background',
                            !isVideoEnabled && 'opacity-100'
                          )}
                        />
                        <LuPause
                          className={cn(
                            'absolute fill-foreground stroke-foreground opacity-0 transition-opacity',
                            'group-hover:fill-background group-hover:stroke-background',
                            isVideoEnabled && 'opacity-100'
                          )}
                        />
                      </Button>
                    </div>
                  </div>
                </div>
              ))}
            </Slideshow>
            <div className="relative -top-16 z-20 space-y-8">
              {tvTrending && (
                <TvCarousel items={tvTrending} header="Trending Tv shows" />
              )}
              {movieTrending && (
                <MovieCarousel items={movieTrending} header="Trending Movies" />
              )}
            </div>
          </>
        )}
      </div>
    </>
  )
}
