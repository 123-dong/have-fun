import { useEffect, useState } from "react";
import { fetchUsers } from "../api/user";
import type { User } from "../api/user";

export default function UserList() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchUsers()
      .then(setUsers)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div>Loading...</div>;

  return (
    <ul>
      {users.map((user) => (
        <li key={user.id}>
          {user.name} (id: {user.id})
        </li>
      ))}
    </ul>
  );
}
