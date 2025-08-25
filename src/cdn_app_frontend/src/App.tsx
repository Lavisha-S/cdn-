import React, { useEffect, useState } from 'react'
import { listFiles, uploadFile, getFile, deleteFile, whoami, login, register, logout, grantRole, revokeRole, listAllUserRoles, getConfig, updateConfig, resetConfig, wipeAll } from './api/actor'

function bytesToBlobUrl(arr: number[], filename: string){
  const u8 = new Uint8Array(arr)
  const blob = new Blob([u8])
  return URL.createObjectURL(blob)
}

export default function App(){
  const [files, setFiles] = useState<any[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [toast, setToast] = useState<string | null>(null)
  const [who, setWho] = useState<string>('')
  const [rolesMap, setRolesMap] = useState<any>(null)
  const [config, setConfig] = useState<any>(null)
  const [page, setPage] = useState(0)
  const [pageSize, setPageSize] = useState(10)

  async function refresh(){
    setError(null)
    try{
      setLoading(true)
      const r = await listFiles()
      if(r.ok) setFiles(r.ok)
      else setError(r.err)
    }catch(e:any){ setError(String(e)) }
    setLoading(false)
  }

  useEffect(()=>{ refresh(); }, [])

  // upload handler
  async function onUpload(file: File){
    setError(null)
    try{
      setLoading(true)
      const buf = new Uint8Array(await file.arrayBuffer())
      const res = await uploadFile(file.name, Array.from(buf))
      setLoading(false)
      if(res.ok){ setToast('Uploaded'); refresh() }
      else setError(res.err)
    }catch(e:any){ setLoading(false); setError(String(e)) }
  }

  return (
    <div className="app">
      <header>
        <h1>CDN App</h1>
      </header>

      <main>
        <section className="panel">
          <h2>Files</h2>
          {error && <div className="error">{error}</div>}
          <div className="controls">
            <input type="file" id="fileInput" onChange={(e)=>{ const f = e.currentTarget.files?.[0]; if(f) onUpload(f) }} />
            <button onClick={refresh} disabled={loading}>Refresh</button>
          </div>

          <div className="file-list">
            {files.length === 0 && <div className="muted">No files</div>}
            {files.slice(page*pageSize,(page+1)*pageSize).map((f)=> (
              <div key={f.id} className="file">
                <div>
                  <strong>{f.filename}</strong>
                  <div className="meta">uploaded by {f.uploader} — {new Date(Number(f.uploaded_at)).toLocaleString()}</div>
                </div>
                <div className="controls">
                  <button onClick={async ()=>{
                    const g = await getFile(f.id)
                    if(g.ok){ const url = bytesToBlobUrl(g.ok.content, f.filename); const a = document.createElement('a'); a.href = url; a.download = f.filename; document.body.appendChild(a); a.click(); a.remove(); URL.revokeObjectURL(url); setToast('Downloaded') } else setError(g.err)
                  }}>Download</button>
                  <button className="secondary" onClick={async ()=>{ if(!confirm('Delete?')) return; const d = await deleteFile(f.id); if(d.ok){ setToast('Deleted'); refresh() } else setError(d.err) }}>Delete</button>
                </div>
              </div>
            ))}
          </div>

          <div className="pagination">
            <label>Page size: <select value={pageSize} onChange={e=>{ setPageSize(Number(e.target.value)); setPage(0) }}><option>5</option><option>10</option><option>20</option></select></label>
            <button onClick={()=>setPage(p=>Math.max(0,p-1))}>Prev</button>
            <button onClick={()=>setPage(p=>p+1)}>Next</button>
            <span className="muted">{Math.ceil(files.length / pageSize)} pages</span>
          </div>
        </section>

        <section className="panel">
          <h2>Admin / Auth</h2>
          <div className="admin-grid">
            <div>
              <button onClick={async ()=>{ const r = await whoami(); setWho(r[0] + ' roles=' + JSON.stringify(r[1])); setToast('whoami') }}>Whoami</button>
              <div className="muted">{who}</div>
            </div>

            <div>
              <input id="authName" placeholder="username" />
              <button onClick={async ()=>{ const n = (document.getElementById('authName') as HTMLInputElement).value || 'guest'; const r = await login(n); setToast(JSON.stringify(r)) }}>Login</button>
              <button onClick={async ()=>{ const n = (document.getElementById('authName') as HTMLInputElement).value || 'guest'; const r = await register(n); setToast(JSON.stringify(r)) }}>Register</button>
              <button onClick={async ()=>{ const r = await logout(); setToast('logout') }}>Logout</button>
            </div>

            <div>
              <input id="roleUser" placeholder="user text/principal" />
              <select id="roleSelect"><option value="Viewer">Viewer</option><option value="Admin">Admin</option><option value="Publisher">Publisher</option></select>
              <button onClick={async ()=>{ const u = (document.getElementById('roleUser') as HTMLInputElement).value; const role = (document.getElementById('roleSelect') as HTMLSelectElement).value; const r = await grantRole(u, role); setToast(JSON.stringify(r)) }}>Grant</button>
              <button onClick={async ()=>{ const u = (document.getElementById('roleUser') as HTMLInputElement).value; const role = (document.getElementById('roleSelect') as HTMLSelectElement).value; const r = await revokeRole(u, role); setToast(JSON.stringify(r)) }}>Revoke</button>
            </div>

            <div>
              <button onClick={async ()=>{ const r = await listAllUserRoles(); setRolesMap(r); setToast('roles loaded') }}>List Roles</button>
              <pre className="muted">{rolesMap ? JSON.stringify(rolesMap, null, 2) : ''}</pre>
            </div>

            <div>
              <button onClick={async ()=>{ const r = await getConfig(); setConfig(r); setToast('config loaded') }}>Get Config</button>
              <button onClick={async ()=>{ if(!confirm('Reset config?')) return; const r = await resetConfig(); setConfig(r); setToast('config reset') }}>Reset</button>
              <div className="muted">{config ? JSON.stringify(config) : ''}</div>
            </div>

            <div>
              <button onClick={async ()=>{ if(!confirm('Wipe all files?')) return; const r = await wipeAll(); setToast('wiped'); refresh() }}>Wipe All</button>
            </div>

          </div>
        </section>
      </main>

      <footer className="toast">{toast}</footer>
    </div>
  )
}
