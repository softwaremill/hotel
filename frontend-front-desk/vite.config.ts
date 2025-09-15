import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { VitePWA } from 'vite-plugin-pwa'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true,
        type: 'module'
      },
      includeAssets: ['favicon.ico', 'apple-touch-icon.png', 'mask-icon.svg'],
      manifest: {
        name: 'Hotel Front Desk',
        short_name: 'Hotel FD',
        description: 'Hotel Front Desk Management System',
        theme_color: '#ffffff',
        background_color: '#ffffff',
        display: 'standalone',
        icons: [
          {
            src: 'pwa-192x192.png',
            sizes: '192x192',
            type: 'image/png'
          },
          {
            src: 'pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png'
          }
        ]
      },
      workbox: {
        // Cache all static assets (app shell)
        globPatterns: ['**/*.{js,css,html,ico,png,svg}'],
        // Cache static API requests only (not Electric SQL streams)
        runtimeCaching: [
          {
            urlPattern: /^http:\/\/localhost:3000\/hotels$/,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'hotels-cache',
              expiration: {
                maxEntries: 10,
                maxAgeSeconds: 300 // 5 minutes
              }
            }
          },
          {
            urlPattern: /^http:\/\/localhost:3000\/hotels\/\d+$/,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'hotel-details-cache',
              expiration: {
                maxEntries: 10,
                maxAgeSeconds: 300
              }
            }
          }
          // Note: Electric SQL shape endpoints are not cached as they deliver incremental data
          // The final computed booking data is cached in the component instead
        ]
      }
    })
  ],
})
