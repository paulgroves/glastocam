# TanStack Start Starter App

A starter application built with [TanStack Start](https://tanstack.com/start), a type-safe, client-first, full-stack React framework.

## Features

- ⚡️ Full-stack React with Server Functions
- 🚦 File-based routing with TanStack Router
- 🔒 Type-safe from database to UI
- 📦 Built with Vite
- 🎨 Styled with Tailwind CSS

## Getting Started

### Prerequisites

- Node.js >= 22.12.0

### Installation

```bash
npm install
```

### Development

Start the development server:

```bash
npm run dev
```

The app will be available at http://localhost:3000

### Production Build

Build and start for production:

```bash
npm run build
npm run start
```

## Project Structure

```
├── src/
│   ├── components/     # Reusable React components
│   ├── routes/         # File-based routes
│   ├── styles/         # CSS styles
│   ├── utils/          # Utility functions
│   ├── router.tsx      # Router configuration
│   └── routeTree.gen.ts # Auto-generated route tree
├── public/             # Static assets
├── vite.config.ts      # Vite configuration
└── package.json
```

## License

MIT

