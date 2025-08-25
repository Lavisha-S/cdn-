// Ambient module declarations for editor/TS-server convenience
declare module '@vitejs/plugin-react'
declare module 'https://cdn.jsdelivr.net/npm/@dfinity/agent@0.11.0/+esm'

declare global {
  interface Window {
    __DFX_CANISTER_ID__?: Record<string, string>
  }
}

export {}
