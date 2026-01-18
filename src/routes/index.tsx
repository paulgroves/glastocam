import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
  component: Home,
})

function Home() {
  return (
    <div className="min-h-screen bg-slate-900 flex flex-col items-center justify-center p-6">
      <h1 className="text-4xl md:text-6xl font-bold text-white mb-4">
        Glastocam
      </h1>
      <p className="text-xl text-gray-300 mb-8 text-center">
        Fullscreen version of Glasto webcam
      </p>
      <p className="text-green-400 text-lg">
        Love the farm, leave no trace 💚
      </p>
    </div>
  )
}

