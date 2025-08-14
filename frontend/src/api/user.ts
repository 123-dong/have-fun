export interface User {
  id: number;
  name: string;
}

export async function fetchUsers(): Promise<User[]> {
  const res = await fetch("http://127.0.0.1:3000/users"); // gateway Axum
  if (!res.ok) throw new Error("Failed to fetch users");
  return res.json();
}

// get by id
export async function fetchUser(id: number): Promise<User> {
  const res = await fetch(`http://127.0.0.1:3000/user?id=${id}`);
  if (!res.ok) throw new Error("Failed to fetch user");
  return res.json();
}
