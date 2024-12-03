// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function debounce<T extends (...args: any[]) => void>(
  func: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null

  return (...args: Parameters<T>): void => {
    if (timeout) {
      clearTimeout(timeout)
    }
    timeout = setTimeout(() => {
      func(...args)
    }, delay)
  }
}

export function formatDuration(seconds: number) {
  const date = new Date(seconds * 1000)
  const hh = date.getUTCHours()
  const mm = date.getUTCMinutes()
  const ss = date.getUTCSeconds()

  if (hh) {
    return `${hh}:${mm.toString().padStart(2, '0')}:${ss
      .toString()
      .padStart(2, '0')}`
  }

  return `${mm}:${ss.toString().padStart(2, '0')}`
}
