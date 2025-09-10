export type User = { id: string; name: string; email: string };

const API_URL = "http://localhost:3001/users";

const handleResponse = async <T>(res: Response): Promise<T> => {
  if (!res.ok) throw new Error(`${res.status}`);
  return res.json();
};

export const fetchUsers = async (): Promise<User[]> =>
  (await handleResponse<{ users: User[] }>(await fetch(API_URL))).users;

export const createUser = async (user: Partial<User>) =>
  handleResponse<User>(
    await fetch(API_URL, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: user.name, email: user.email }),
    })
  );

export const updateUser = async (user: User) =>
  handleResponse<User>(
    await fetch(`${API_URL}/${user.id}`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(user),
    })
  );

export const deleteUser = async (id: string) =>
  handleResponse<void>(await fetch(`${API_URL}/${id}`, { method: "DELETE" }));

export const subscribeUsersSSE = (
  onMessage: (user: User) => void,
  { onOpen, onError }: { onOpen?: () => void; onError?: () => void } = {}
) => {
  const source = new EventSource(`${API_URL}/stream`);
  source.onmessage = (e) =>
    e.data !== "keep-alive" && onMessage(JSON.parse(e.data));
  source.onopen = () => onOpen?.();
  source.onerror = () => onError?.();
  return () => source.close();
};
