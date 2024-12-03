import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useMutation, useQuery } from '@tanstack/react-query'
import api from '@/api'
import { useEffect, useState } from 'react'
import ReactPlayer from 'react-player/lazy'

export const Route = createFileRoute('/watch/')({
  component: Player
})

function Player() {
  // const [playlistUrl, setPlaylistUrl] = useState(null)
  // const [magnet, setMagnet] = useState(null)

  // const { mutate: createPlaylist, isPending } = useMutation({
  //   mutationKey: ['player', magnet],
  //   mutationFn: async () => {
  //     return (
  //       await api.addTorrentWithMagnet({
  //         magnet
  //       })
  //     ).data
  //   },
  //   onSuccess: (data) => {
  //     setPlaylistUrl(`${}`)
  //   }
  // })

  // const loadPlaylist = () => {
  //   createPlaylist()
  // }

  // if (!magnet || !playlistUrl) {
  //   return (
  //     <div className="w-screen h-screen flex items-center justify-center">
  //       <input
  //         type="text"
  //         placeholder="Magnet link"
  //         onChange={(e) => setMagnet(e.target.value)}
  //       />
  //       <button onClick={loadPlaylist} disabled={isPending}>
  //         Load
  //       </button>
  //     </div>
  //   )
  // }

  // if (isPending) {
  //   return <div>Loading...</div>
  // }

  return (
    <div className="w-screen h-screen flex items-center justify-center">
      <ReactPlayer
        url="http://localhost:3000/api/transcode/start.mpd?session_id=test"
        controls={true}
        playing={true}
        width="100%"
        height="100%"
      />
    </div>
  )
}
