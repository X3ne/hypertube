export function getTmdbImageUrl(path?: string | null) {
  if (!path) return ''
  return `https://image.tmdb.org/t/p/original${path}`
}

export function getTmdbPlaceholderImageUrl(
  path?: string | null,
  size = 'w200'
) {
  if (!path) return ''
  return `https://image.tmdb.org/t/p/${size}${path}`
}
