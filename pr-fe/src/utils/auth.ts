import type { UserData } from "../types/prtypes.type";

const LOCALSTORAGE_USER = "pr-user";

export const setLoggedUser = (user: UserData) => {
  localStorage.setItem(LOCALSTORAGE_USER, JSON.stringify(user));
}

export const getLoggedUser = (): UserData => {
  return JSON.parse(localStorage.getItem(LOCALSTORAGE_USER));
}

export const clearLoggedUser = () => {
  localStorage.setItem(LOCALSTORAGE_USER, null);
}

export const isUserLogged = (): boolean => {
  return localStorage.getItem(LOCALSTORAGE_USER) !== null;
}