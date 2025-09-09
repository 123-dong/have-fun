const API_URL = import.meta.env.VITE_API_URL || "http:/localhost:3001";

export type User = {
  id: string;
  name: string;
  email: string;
};

export async function listUsers(): Promise<User[]> {
  const resp = await fetch(`${API_URL}/users`);

  if (!resp.ok) throw new Error(`HTTP error! status: ${resp.status}`);

  const data = await resp.json();

  if (Array.isArray(data)) {
    return data;
  } else if (data.users) {
    return data.users;
  } else {
    throw new Error("Unexpected API response format");
  }
}

export async function streamUsers(): Promise<User[]> {
  const resp = await fetch(`${API_URL}/users/stream`);

  if (!resp.ok) throw new Error(`HTTP error! status: ${resp.status}`);

  const data = await resp.json();

  if (Array.isArray(data)) {
    return data;
  } else if (data.users) {
    return data.users;
  } else {
    throw new Error("Unexpected API response format");
  }
}
