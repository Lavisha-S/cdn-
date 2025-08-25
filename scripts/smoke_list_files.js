import { Actor, HttpAgent } from 'https://cdn.jsdelivr.net/npm/@dfinity/agent@0.11.0/+esm'
import { idlFactory } from '../src/cdn_app_frontend/dist/declarations/cdn_app_backend/cdn_app_backend.did.js'

const CANISTER_ID = process.env.CANISTER_ID || 'ulvla-h7777-77774-qaacq-cai'
const agent = new HttpAgent({ host: 'http://127.0.0.1:4943' })
await agent.fetchRootKey().catch(()=>{})
const actor = Actor.createActor(idlFactory, { agent, canisterId: CANISTER_ID })

const res = await actor.list_files()
console.log('list_files ->', res)
