import React, { useEffect, useState } from 'react'
import { getBackendActor } from '../services/actor'

type FileInfo = { id: string; filename: string; uploader: string; uploaded_at: bigint }

export default function FileManager(){
  const [files, setFiles] = useState<FileInfo[]>([])
  const [loading, setLoading] = useState(false)
  const [fileToUpload, setFileToUpload] = useState<File | null>(null)

  const actor = getBackendActor()
  const [error, setError] = useState<string | null>(null)

  async function loadFiles(){
    if(!actor){ setError('Backend actor not available'); return }
    setLoading(true)
    try{
      const res = await actor.list_files()
      if(res.ok){
        setFiles(res.ok as unknown as FileInfo[])
        setError(null)
      } else {
        console.error('list_files err', res.err)
        setError(res.err ?? 'Unknown error')
      }
    }catch(e:any){
      console.error(e)
      setError(String(e))
    }finally{ setLoading(false) }
  }

  useEffect(()=>{ loadFiles() }, [])

  async function onUpload(e: React.FormEvent){
  e.preventDefault()
  if(!actor){ setError('Backend actor not available'); return }
  if(!fileToUpload) return
  const data = new Uint8Array(await fileToUpload.arrayBuffer())
  const res = await actor.upload_file(fileToUpload.name, Array.from(data))
  if(res.ok){ loadFiles() }
  else { console.error('upload err', res.err); setError(res.err ?? 'Upload failed') }
  }

  async function onDelete(id:string){
  if(!actor){ setError('Backend actor not available'); return }
  const res = await actor.delete_file(id)
  if(res.ok){ loadFiles() }
  else { console.error('delete err', res.err); setError(res.err ?? 'Delete failed') }
  }

  return (
    <div>
      <h2>Files</h2>
  {error ? <div style={{color:'red'}}>Error: {error}</div> : loading ? <div>Loading...</div> : (
        <ul>
          {files.map(f=> (
            <li key={f.id}>
              <strong>{f.filename}</strong> — uploaded by {f.uploader} — <button onClick={()=>onDelete(f.id)}>Delete</button>
            </li>
          ))}
        </ul>
      )}

      <form onSubmit={onUpload}>
        <input type="file" onChange={ev=> setFileToUpload(ev.target.files?.[0] ?? null)} />
        <button type="submit">Upload</button>
      </form>
    </div>
  )
}
