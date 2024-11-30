import quantize from 'quantize'

export type RGB = {
  r: number
  g: number
  b: number
}

export const toRgb = (color: RGB) => `rgb(${color.r}, ${color.g}, ${color.b})`

export const toRgba = (color: RGB, alpha: number) => {
  return `rgba(${color.r}, ${color.g}, ${color.b}, ${alpha})`
}
/**
 * Extract dominant colors from an image using color quantization.
 * @param imageUrl URL of the image to extract colors from.
 * @param colorCount Number of dominant colors to return.
 * @returns Promise that resolves to an array of dominant colors in RGB format.
 */
export async function extractColors(
  imageUrl: string,
  colorCount: number = 5
): Promise<RGB[]> {
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.crossOrigin = 'Anonymous'
    img.src = imageUrl

    img.onload = () => {
      const canvas = document.createElement('canvas')
      canvas.width = img.width
      canvas.height = img.height
      const context = canvas.getContext('2d')

      if (!context) {
        reject('Could not get canvas context')
        return
      }

      context.drawImage(img, 0, 0, img.width, img.height)

      const imageData = context.getImageData(0, 0, img.width, img.height)
      const data = imageData.data

      const pixels: [number, number, number][] = []

      for (let i = 0; i < data.length; i += 4) {
        const r = data[i]
        const g = data[i + 1]
        const b = data[i + 2]
        pixels.push([r, g, b])
      }

      const colorMap = quantize(pixels, colorCount)
      const dominantColors = colorMap ? colorMap.palette() : []

      const rgbColors = dominantColors.map((color) => ({
        r: color[0],
        g: color[1],
        b: color[2]
      }))

      resolve(rgbColors)
    }

    img.onerror = () => {
      reject('Failed to load the image')
    }
  })
}
