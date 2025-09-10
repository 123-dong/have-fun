import { useUsers } from "../hooks/useUsers";
import UserTable from "../components/UserTable";
import UserForm from "../components/UserFrom";
import StreamStatus from "../components/StreamStatus";

export default function Home() {
  const {
    users,
    selectedUser,
    streamOn,
    streamStatus,
    toggleStream,
    saveUser,
    deleteUser,
    setSelectedUser,
  } = useUsers();

  return (
    <div className="p-6 space-y-6">
      <h1 className="text-2xl font-bold">User Dashboard</h1>

      <StreamStatus
        streamOn={streamOn}
        streamStatus={streamStatus}
        onToggle={toggleStream}
      />

      <div className="flex space-x-6 mt-4">
        <div className="w-1/3">
          <UserForm
            selectedUser={selectedUser}
            onSave={saveUser}
            onDelete={deleteUser}
          />
        </div>
        <div className="w-2/3">
          <UserTable
            users={users}
            onSelect={setSelectedUser}
            onDelete={deleteUser}
          />
        </div>
      </div>
    </div>
  );
}
