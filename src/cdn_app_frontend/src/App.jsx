import { useState } from "react";
import { cdn_app_backend } from "../../declarations/cdn_app_backend";

export default function App() {
  const [file, setFile] = useState(null);
  const [url, setUrl] = useState("");

  async function upload() {
    if (!file) return;
    const reader = new FileReader();
    reader.onload = async (e) => {
      const data = Array.from(new Uint8Array(e.target.result));
      const id = crypto.randomUUID(); // unique ID
      await cdn_app_backend.upload_file(id, file.name, file.type, data);
      const canisterId = process.env.CANISTER_ID_CDN_APP_BACKEND;
      setUrl(`https://${canisterId}.raw.icp0.io/file/${id}`);
    };
    reader.readAsArrayBuffer(file);
  }

  return (
    <div className="p-4">
      <h1>CDN Uploader</h1>
      <input type="file" onChange={(e) => setFile(e.target.files[0])} />
      <button onClick={upload}>Upload</button>
      {url && <p>File URL: <a href={url} target="_blank">{url}</a></p>}
    </div>
  );
}
