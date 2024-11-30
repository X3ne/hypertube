import { useEffect, useState } from 'react'
import { useGradient } from '@/hooks/useGradient'
import { RGB, toRgb, toRgba } from '@/utils/extractColor'

const generateGradient = (colors: RGB[], darkness: number = 0) => {
  if (!colors.length) return ''

  const darken = (color: RGB, darkness: number) => ({
    r: Math.max(0, color.r - darkness),
    g: Math.max(0, color.g - darkness),
    b: Math.max(0, color.b - darkness)
  })

  return `
      radial-gradient(circle farthest-side at 0% 100%, ${toRgb(darken(colors[0], darkness))} 0%, ${toRgba(darken(colors[0], darkness), 0)} 100%),
      radial-gradient(circle farthest-side at 100% 100%, ${toRgb(darken(colors[2], darkness))} 0%, ${toRgba(darken(colors[2], darkness), 0)} 100%),
      radial-gradient(circle farthest-side at 100% 0%, ${toRgb(darken(colors[3], darkness))} 0%, ${toRgba(darken(colors[3], darkness), 0)} 100%),
      radial-gradient(circle farthest-side at 0% 0%, ${toRgb(darken(colors[1], darkness))} 0%, ${toRgba(darken(colors[1], darkness), 0)} 100%),
      black
    `
}

const Gradient = () => {
  const { image, colors } = useGradient()

  const [gradient, setGradient] = useState({
    colors: [] as RGB[],
    before: '',
    after: '',
    opacity: '0'
  })

  useEffect(() => {
    if (colors.length === 0) {
      setGradient((prev) => ({ ...prev, colors: [], before: '', after: '' }))
    }

    setGradient((prev) => ({ ...prev, opacity: '0' }))

    const timeout = setTimeout(() => {
      setGradient((prev) => ({
        ...prev,
        colors,
        after: generateGradient(colors, image ? 70 : 0),
        before: prev.after || prev.before,
        opacity: '100'
      }))
    }, 300)

    return () => clearTimeout(timeout)
  }, [colors])

  return (
    <>
      {colors.length > 0 && (
        <div className="fixed bottom-0 left-0 flex w-screen h-2 z-50">
          {colors.map((color, index) => (
            <div
              key={index}
              className="w-1/4"
              style={{ background: toRgb(color) }}
            />
          ))}
        </div>
      )}

      <div className="fixed w-screen h-screen top-0 left-0 -z-20">
        <div
          className="fixed w-full h-full transition-opacity duration-1000"
          style={{
            background: gradient.before,
            opacity: gradient.opacity === '100' ? '0' : '100',
            position: 'absolute',
            top: 0,
            left: 0,
            zIndex: -2
          }}
        ></div>

        <div
          className="fixed w-full h-full transition-opacity duration-1000"
          style={{
            background: gradient.after,
            opacity: gradient.opacity,
            position: 'absolute',
            top: 0,
            left: 0,
            zIndex: -1
          }}
        ></div>

        <div className="fixed backdrop-blur-2xl bg-secondary/70 w-full h-full"></div>
      </div>
    </>
  )
}

export default Gradient
