import { useState, useEffect } from "react";
import type { User } from "../api/client";

type Props = {
  selectedUser: User | null;
  onSave: (user: Partial<User>) => void;
  onDelete: (id: string) => void;
};

export default function UserForm({ selectedUser, onSave, onDelete }: Props) {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");

  useEffect(() => {
    if (selectedUser) {
      setName(selectedUser.name);
      setEmail(selectedUser.email);
    } else {
      setName("");
      setEmail("");
    }
  }, [selectedUser]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!name.trim() || !email.trim())
      return alert("Name and Email are required");

    // Email regex nhẹ, chấp nhận abc@abc
    if (!/^\S+@\S+$/.test(email)) return alert("Email format invalid");

    if (selectedUser) {
      onSave({ id: selectedUser.id, name, email });
    } else {
      onSave({ name, email });
    }
  };

  return (
    <form
      className="p-4 bg-white rounded shadow space-y-3"
      onSubmit={handleSubmit}
    >
      <input
        type="text"
        placeholder="Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
        className="border p-2 w-full"
      />
      <input
        type="email"
        placeholder="Email"
        value={email}
        onChange={(e) => setEmail(e.target.value)}
        className="border p-2 w-full"
      />
      <div className="flex space-x-2">
        <button
          type="submit"
          className="bg-blue-500 text-white px-4 py-2 rounded"
        >
          {selectedUser ? "Update" : "Create"}
        </button>
        {selectedUser && (
          <button
            type="button"
            className="bg-red-500 text-white px-4 py-2 rounded"
            onClick={() => onDelete(selectedUser.id)}
          >
            Delete
          </button>
        )}
      </div>
    </form>
  );
}
