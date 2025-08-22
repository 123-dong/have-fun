// src/App.tsx
import { useState } from "react";

function App() {
  const [query, setQuery] = useState("");
  const [result, setResult] = useState<any>(null);

  const callApi = async (endpoint: string) => {
    try {
      const res = await fetch(endpoint);
      const data = await res.json();
      setResult(data);
    } catch (err) {
      setResult({ error: "API request failed" });
    }
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col items-center p-6">
      <h1 className="text-2xl font-bold mb-4">API Tester</h1>

      <div className="flex gap-2 mb-4 w-full max-w-md">
        <input
          type="text"
          placeholder="Enter query..."
          className="flex-1 border border-gray-700 bg-gray-800 text-gray-100 p-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        <button
          onClick={() => callApi(`/api/search?q=${query}`)}
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg"
        >
          Search
        </button>
      </div>

      <div className="flex gap-2 mb-6">
        <button
          onClick={() => callApi("/api/users")}
          className="bg-green-600 hover:bg-green-700 text-white px-4 py-2 rounded-lg"
        >
          Get Users
        </button>
        <button
          onClick={() => callApi("/api/posts")}
          className="bg-purple-600 hover:bg-purple-700 text-white px-4 py-2 rounded-lg"
        >
          Get Posts
        </button>
      </div>

      <pre className="bg-gray-800 p-4 rounded-lg shadow max-w-2xl w-full overflow-auto text-sm">
        {result ? JSON.stringify(result, null, 2) : "No data yet"}
      </pre>
    </div>
  );
}

export default App;
