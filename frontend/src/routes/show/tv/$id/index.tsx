import LazyImage from '@/components/lazy-image'
import { useGradient } from '@/hooks/useGradient'
import { useSong } from '@/hooks/useSong'
import { Marika } from '@shineiichijo/marika'
import { useQuery } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { useEffect } from 'react'
import createDOMPurify from 'dompurify'
import {
  LuBookmark,
  LuCalendar,
  LuClapperboard,
  LuHeart,
  LuPlay,
  LuStar,
  LuTrendingUp
} from 'react-icons/lu'
import { Button } from '@/components/ui/button'
import api from '@/api'
import { getTmdbImageUrl, getTmdbPlaceholderImageUrl } from '@/utils/tmdb'
import { Link } from '@tanstack/react-router'
import { cn } from '@/lib/utils'
import Separator from '../../../../components/ui/separator'

export const Route = createFileRoute('/show/tv/$id/')({
  component: ShowView
})

function ShowView() {
  const { id } = Route.useParams()
  const { changeImage } = useGradient()
  const { changeShow } = useSong()

  const DOMPurify = createDOMPurify(window)

  const { data: show, isLoading: isShowLoading } = useQuery({
    queryKey: ['show', id],
    queryFn: async () => {
      return (await api.getTv(Number(id))).data
    },
    refetchOnWindowFocus: false
  })

  useEffect(() => {
    changeImage(getTmdbPlaceholderImageUrl(show?.backdrop_path) || null)
    changeShow(show || null)
  }, [show])

  return (
    <div>
      {isShowLoading || !show ? (
        <p>Loading...</p>
      ) : (
        <div>
          <div className="absolute top-0 left-0 w-full h-52 md:h-96 -z-10">
            <LazyImage
              src={getTmdbImageUrl(show.backdrop_path) || ''}
              alt={show.name}
              className="w-full h-full object-cover"
            />
            <div className=" bg-gradient-to-t from-background/80 to-transparent absolute inset-0" />
          </div>
          <div
            className={cn(
              'flex flex-col gap-6 w-full mt-28 px-4',
              'md:mt-48 md:px-24'
            )}
          >
            <div className="flex flex-col gap-4 w-full md:flex-row md:gap-8">
              <LazyImage
                src={getTmdbImageUrl(show.poster_path) || ''}
                alt={show.name}
                className="w-64 h-96 mx-auto rounded-md object-cover"
              />

              <div className="flex-grow space-y-4 md:self-end">
                <h1 className="text-3xl font-semibold">{show.name}</h1>

                <div className="flex gap-1 flex-wrap w-full">
                  <div className="flex items-center gap-2 pr-4">
                    <LuTrendingUp className="w-5 h-5 inline" />
                    <span>Rating: {show.vote_average}</span>
                  </div>
                  <div className="flex items-center gap-2 pr-4">
                    <LuCalendar className="w-5 h-5 inline" />
                    <span>
                      Year: {new Date(show.first_air_date).getFullYear()}
                    </span>
                  </div>
                  <div className="flex items-center gap-2 pr-4">
                    <LuClapperboard className="w-5 h-5 inline" />
                    <span>Episodes: {show.number_of_episodes}</span>
                  </div>
                </div>

                <div className="flex gap-2">
                  {/* TODO: when watching add next episode or resume button */}
                  <Button shape={'square'} className="gap-2 px-12">
                    <LuPlay className="w-4 h-4 inline" fill="black" />
                    Watch now
                  </Button>
                  <Button variant={'outline'} shape={'square'} size={'icon'}>
                    <LuHeart className="w-4 h-4 inline" />
                  </Button>
                  <Button variant={'outline'} shape={'square'} size={'icon'}>
                    <LuBookmark className="w-4 h-4 inline" />
                  </Button>
                </div>
              </div>
            </div>

            <div>
              {show.genres && (
                <div className="flex gap-2 flex-wrap">
                  {show.genres.map((genre) => (
                    <span
                      key={genre.id}
                      className="px-2 py-1 bg-accent rounded-md text-accent-foreground"
                    >
                      {genre.name}
                    </span>
                  ))}
                </div>
              )}

              <div>
                <Separator header="Synopsis" />
                <p
                  className="line-clamp-[10] text-sm mt-4 text-foreground/80 hover:text-foreground transition-colors duration-500"
                  dangerouslySetInnerHTML={{
                    __html: DOMPurify.sanitize(show.overview)
                  }}
                />
              </div>
            </div>
          </div>

          <div className="flex gap-4 px-12 py-6">
            {show.seasons &&
              show.seasons.map((season) => (
                <Link
                  key={season.id}
                  to={`/show/tv/${id}/season/${season.season_number}`}
                  className="flex flex-col gap-2"
                >
                  <div className="w-52 h-72 rounded-md overflow-hidden">
                    <LazyImage
                      src={
                        season.poster_path
                          ? getTmdbImageUrl(season.poster_path)
                          : getTmdbImageUrl(show.poster_path) || ''
                      }
                      alt={season.name}
                      className="hover:scale-105 transition-transform duration-500"
                    />
                  </div>
                  <div className="grow">
                    <h1>{season.name}</h1>
                  </div>
                </Link>
              ))}
          </div>
        </div>
      )}
    </div>
  )
}
