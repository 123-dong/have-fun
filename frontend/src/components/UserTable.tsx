import type { User } from "../api/client";

type Props = {
  users: User[];
  onSelect: (user: User) => void;
  onDelete: (id: string) => void;
};

export default function UserTable({ users, onSelect, onDelete }: Props) {
  return (
    <table className="min-w-full border">
      <thead className="bg-gray-200">
        <tr>
          <th className="p-2 border">ID</th>
          <th className="p-2 border">Name</th>
          <th className="p-2 border">Email</th>
          <th className="p-2 border">Actions</th>
        </tr>
      </thead>
      <tbody>
        {users.map((user) => (
          <tr key={user.id} className="hover:bg-gray-100">
            <td className="p-2 border">{user.id}</td>
            <td className="p-2 border">{user.name}</td>
            <td className="p-2 border">{user.email}</td>
            <td className="p-2 border flex space-x-2">
              <button
                className="bg-green-500 text-white px-2 py-1 rounded"
                onClick={() => onSelect(user)}
              >
                Edit
              </button>
              <button
                className="bg-red-500 text-white px-2 py-1 rounded"
                onClick={() => onDelete(user.id)}
              >
                Delete
              </button>
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}
