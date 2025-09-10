import { useState, useEffect } from "react";
import { userService } from "../services/userService";
import type { User } from "../api/client";

type StreamStatus = "idle" | "connecting" | "connected" | "failed";

export function useUsers() {
  const [users, setUsers] = useState<User[]>([]);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [streamOn, setStreamOn] = useState(false);
  const [streamStatus, setStreamStatus] = useState<StreamStatus>("idle");
  const [unsubscribeStream, setUnsubscribeStream] = useState<() => void>();

  // fetch initial users
  useEffect(() => {
    if (!streamOn) {
      userService.fetchUsers().then(setUsers).catch(console.error);
    }
  }, [streamOn]);

  const toggleStream = () => {
    if (streamOn) {
      unsubscribeStream?.();
      setUnsubscribeStream(undefined);
      setStreamOn(false);
      setStreamStatus("idle");
    } else {
      setStreamStatus("connecting");
      const unsubscribe = userService.subscribeUsers(
        (user) => {
          setUsers((prev) => {
            const idx = prev.findIndex((u) => u.id === user.id);
            if (idx >= 0) {
              const prevUser = prev[idx];
              if (prevUser.name === user.name && prevUser.email === user.email)
                return prev;
              const updated = [...prev];
              updated[idx] = user;
              return updated;
            }
            return [...prev, user];
          });
          setStreamStatus("connected");
        },
        {
          onOpen: () => setStreamStatus("connected"),
          onError: () => setStreamStatus("failed"),
        }
      );
      setUnsubscribeStream(() => unsubscribe);
      setStreamOn(true);
    }
  };

  const saveUser = async (user: Partial<User>) => {
    if (user.id) await userService.updateUser(user as User);
    else await userService.createUser(user);
    setSelectedUser(null);
  };

  const deleteUser = async (id: string) => {
    await userService.deleteUser(id);
    setUsers((prev) => prev.filter((u) => u.id !== id));
    if (selectedUser?.id === id) setSelectedUser(null);
  };

  return {
    users,
    selectedUser,
    streamOn,
    streamStatus,
    toggleStream,
    saveUser,
    deleteUser,
    setSelectedUser,
  };
}
