import type { UserData } from '../types/prtypes.type';

const API: string = "http://localhost:7005";
const EVENT_API: string = `${API}/events`;
const USER_API: string = `${API}/users`;
const REFDATA_API: string = `${API}/ref_data`;


type Options = {
  method: string,
  headers: {
    "Content-Type": string
  }
}

const GET_OPTIONS: Options = {
  method: "GET",
  headers: {
    "Content-Type": "application/json"
  }
}

const POST_OPTIONS: Options = {
  method: "POST",
  headers: {
    "Content-Type": "application/json"
  }
}

const PUT_OPTIONS: Options = {
  method: "PUT",
  headers: {
    "Content-Type": "application/json"
  }
}

const DELETE_OPTIONS: Options = {
  method: "DELETE",
  headers: {
    "Content-Type": "application/json"
  }
}

export const createEvent = async (name: string) => {
  let r = await fetch(EVENT_API, { ...POST_OPTIONS, body: JSON.stringify({ name })});
  return r.json();
}

export const addUserToEvent = async (id: string, data: UserData) => {
  await fetch(`${EVENT_API}/${id}/participants`, { ...POST_OPTIONS, body: JSON.stringify(data)});
}

export const removeUserFromEvent = async (eventId: string, userId: string) => {
  await fetch(`${EVENT_API}/${eventId}/participants/${userId}`, DELETE_OPTIONS);
}

export const deleteEvent = async (id: string) => {
  await fetch(`${EVENT_API}/${id}`, DELETE_OPTIONS);
}

export const getEvents = async () => {
  let r = await fetch(EVENT_API, GET_OPTIONS);
  return r.json();
}

export const createUser = async (name: string, dispositions: Array<string>) => {
  let r = await fetch(USER_API, { ...POST_OPTIONS, body: JSON.stringify({ name, dispositions })});
  return r.json();
}

export const deleteUser = async (id: string) => {
  await fetch(`${USER_API}/${id}`, DELETE_OPTIONS);
}

export const updateUser = async (id: string, data: UserData) => {
  await fetch(`${USER_API}/${id}`, { ...PUT_OPTIONS, body: JSON.stringify({ name: data.name, dispositions: data.dispositions })});
}

export const getUsers = async () => {
  let r = await fetch(USER_API, GET_OPTIONS);
  return r.json();
}

export const getReferenceData = async () => {
  let r = await fetch(REFDATA_API, GET_OPTIONS);
  return r.json();
}
