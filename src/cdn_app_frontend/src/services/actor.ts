import { createActor as createBackendActor, cdn_app_backend } from '../../declarations/cdn_app_backend';

export function getBackendActor() {
  // Prefer the already-initialized actor (cdn_app_backend) if present.
  if (typeof cdn_app_backend !== 'undefined' && cdn_app_backend) {
    return cdn_app_backend;
  }

  // Fallback: use createActor with the Vite-provided canister id.
  const canisterId = (process && (process.env as any)?.VITE_BACKEND_CANISTER_ID) || undefined;
  try {
    return createBackendActor(canisterId);
  } catch (e) {
    console.error('Failed to create backend actor', e);
    return undefined;
  }
}
