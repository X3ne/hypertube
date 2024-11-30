import LazyImage from '@/components/lazy-image'
import { useGradient } from '@/hooks/useGradient'
import { useSong } from '@/hooks/useSong'
import { Marika } from '@shineiichijo/marika'
import { useQuery } from '@tanstack/react-query'
import { createFileRoute } from '@tanstack/react-router'
import { useEffect } from 'react'
import createDOMPurify from 'dompurify'
import { LuHeart, LuStar } from 'react-icons/lu'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import api from '@/api'
import { getTmdbImageUrl, getTmdbPlaceholderImageUrl } from '@/utils/tmdb'
import { Link } from '@tanstack/react-router'

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
  }, [show, changeImage, changeShow])

  return (
    <div>
      {isShowLoading || !show ? (
        <p>Loading...</p>
      ) : (
        <div>
          <div className="relative">
            <LazyImage
              src={getTmdbImageUrl(show.backdrop_path) || ''}
              alt={show.name}
              className="w-full h-80"
            />
            <div className="bg-gradient-to-t from-background/50 to-transparent absolute inset-0" />
          </div>
          <div className="w-full -mt-24">
            <div className="flex flex-col gap-2 px-32 pb-6 w-full">
              <div className="flex">
                <div>
                  <LazyImage
                    src={getTmdbImageUrl(show.poster_path) || ''}
                    alt={show.name}
                    className="w-64 h-96 rounded-md"
                  />
                </div>
                <div className="grow mt-24 p-8 pb-0">
                  <h1 className="text-xl">{show.name}</h1>
                  <p
                    className="line-clamp-[10] text-sm mt-4 text-foreground/80 hover:text-foreground transition-colors duration-500"
                    dangerouslySetInnerHTML={{
                      __html: DOMPurify.sanitize(show.overview)
                    }}
                  />
                </div>
              </div>
            </div>
          </div>
          <div className="px-32 mt-4">
            <div className="flex flex-col gap-2">
              <div className="flex justify-center items-center gap-2 px-3 py-1 w-fit rounded-md">
                <LuStar className="w-5 h-5 stroke-yellow-500 fill-yellow-500" />
                <span>Score {show.vote_average.toFixed(1) || '#'}</span>
              </div>
              <div className="flex justify-center items-center gap-2 px-3 py-1 w-fit rounded-md">
                <LuHeart className="w-5 h-5 stroke-red-600 fill-red-600" />
                <span>Popularity {show.popularity.toFixed(1) || '#'}</span>
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
                  <LazyImage
                    src={
                      season.poster_path
                        ? getTmdbImageUrl(season.poster_path)
                        : getTmdbImageUrl(show.poster_path) || ''
                    }
                    alt={season.name}
                    className="w-52 h-72 rounded-md"
                  />
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
