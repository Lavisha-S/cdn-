import React, { useEffect, useState, useRef } from 'react'
import { createActor as createBackendActor, cdn_app_backend } from '../../declarations/cdn_app_backend'

type FileInfo = { id: string; filename: string; uploader: string; uploaded_at: bigint }
type FileContents = { filename: string; content: number[] }

export default function App(){
  const [files, setFiles] = useState<FileInfo[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [fileToUpload, setFileToUpload] = useState<File | null>(null)
  const actorRef = useRef<any | undefined>(undefined)

  // Create or reuse actor
  function getActor(){
    if(actorRef.current) return actorRef.current
    if(typeof cdn_app_backend !== 'undefined' && cdn_app_backend){
      actorRef.current = cdn_app_backend
      return actorRef.current
    }
    const canisterId = (process && (process.env as any)?.VITE_BACKEND_CANISTER_ID) || undefined
    try{
      actorRef.current = createBackendActor(canisterId)
      return actorRef.current
    }catch(e:any){
      console.error('actor create failed', e)
      setError(String(e))
      return undefined
    }
  }

  async function loadFiles(){
    const actor = getActor()
    if(!actor){ setError('Backend actor not available'); return }
    setLoading(true)
    try{
      const res = await actor.list_files()
      if(res.ok){
        setFiles(res.ok as FileInfo[])
        setError(null)
      } else {
        setError(res.err ?? 'list_files failed')
      }
    }catch(e:any){
      setError(String(e))
    }finally{ setLoading(false) }
  }

  useEffect(()=>{ loadFiles() }, [])

  async function onUpload(e: React.FormEvent){
    e.preventDefault()
    const actor = getActor()
    if(!actor){ setError('Backend actor not available'); return }
    if(!fileToUpload) return
    try{
      const bytes = new Uint8Array(await fileToUpload.arrayBuffer())
      const res = await actor.upload_file(fileToUpload.name, Array.from(bytes))
      if(res.ok){ setFileToUpload(null); loadFiles() }
      else setError(res.err ?? 'upload failed')
    }catch(e:any){ setError(String(e)) }
  }

  async function onDelete(id:string){
    const actor = getActor()
    if(!actor){ setError('Backend actor not available'); return }
    try{
      const res = await actor.delete_file(id)
      if(res.ok) loadFiles()
      else setError(res.err ?? 'delete failed')
    }catch(e:any){ setError(String(e)) }
  }

  async function onDownload(id:string, filename:string){
    const actor = getActor()
    if(!actor){ setError('Backend actor not available'); return }
    try{
      const res = await actor.get_file(id)
      if(res.ok){
        const contents = res.ok as FileContents
        const arr = new Uint8Array(contents.content as number[])
        const blob = new Blob([arr])
        const url = URL.createObjectURL(blob)
        const a = document.createElement('a')
        a.href = url
        a.download = filename
        document.body.appendChild(a)
        a.click()
        a.remove()
        URL.revokeObjectURL(url)
      } else {
        setError(res.err ?? 'get_file failed')
      }
    }catch(e:any){ setError(String(e)) }
  }

  return (
    <div style={{padding:20}}>
      <h1>CDN App Frontend</h1>

      {error && <div style={{color:'red', marginBottom:10}}>Error: {error}</div>}

      <section style={{marginBottom:20}}>
        <h2>Files</h2>
        {loading && <div>Loading...</div>}
        {!loading && files.length === 0 && <div>No files found</div>}
        {!loading && files.length > 0 && (
          <ul>
            {files.map(f => (
              <li key={f.id} style={{marginBottom:8}}>
                <strong>{f.filename}</strong>
                <div style={{fontSize:12, color:'#666'}}>uploaded by {f.uploader} — {String(f.uploaded_at)}</div>
                <div style={{marginTop:6}}>
                  <button onClick={()=> onDownload(f.id, f.filename)} style={{marginRight:8}}>Download</button>
                  <button onClick={()=> onDelete(f.id)}>Delete</button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Upload</h2>
        <form onSubmit={onUpload}>
          <input type="file" onChange={ev=> setFileToUpload(ev.target.files?.[0] ?? null)} />
          <button type="submit" style={{marginLeft:8}}>Upload</button>
        </form>
      </section>
    </div>
  )
}
