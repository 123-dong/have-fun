import * as API from "../api/client";

export const userService = {
  fetchUsers: API.fetchUsers,
  createUser: API.createUser,
  updateUser: API.updateUser,
  deleteUser: API.deleteUser,
  subscribeUsers: API.subscribeUsersSSE,
};
