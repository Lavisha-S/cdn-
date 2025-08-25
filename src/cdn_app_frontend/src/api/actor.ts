import { Actor, HttpAgent } from '@dfinity/agent'
import { idlFactory } from '../declarations/cdn_app_backend/cdn_app_backend.did.js'

const CANISTER_ID = (typeof window !== 'undefined' && window.__DFX_CANISTER_ID__ && window.__DFX_CANISTER_ID__.cdn_app_backend) || 'ulvla-h7777-77774-qaacq-cai'
const agent = new HttpAgent({ host: 'http://127.0.0.1:4943' })
agent.fetchRootKey().catch(()=>{})
const actor = Actor.createActor(idlFactory, { agent, canisterId: CANISTER_ID })

export const listFiles = () => actor.list_files()
export const uploadFile = (name: string, bytes: number[]) => actor.upload_file(name, bytes)
export const getFile = (id: string) => actor.get_file(id)
export const deleteFile = (id: string) => actor.delete_file(id)
export const whoami = () => actor.whoami()
export const login = (name: string) => actor.login(name)
export const register = (name: string) => actor.register(name)
export const logout = () => actor.logout()
export const grantRole = (user: string, role: any) => actor.grant_role(user, role)
export const revokeRole = (user: string, role: any) => actor.revoke_role(user, role)
export const listAllUserRoles = () => actor.list_all_user_roles()
export const getConfig = () => actor.get_config()
export const updateConfig = (max?: number|null, uploads?: boolean|null, domain?: string|null|undefined) => actor.update_config(max ?? null, uploads ?? null, domain === undefined ? undefined : (domain === null ? null : domain))
export const resetConfig = () => actor.reset_config()
export const wipeAll = () => actor.wipe_all()
