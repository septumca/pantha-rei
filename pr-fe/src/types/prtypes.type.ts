export type UserData = {
  _id: string,
  name: string,
  dispositions: Array<string>
}

export type EventData = {
  _id: string,
  name: string,
  participants: Array<UserData>
}

export type ReferenceData = {
  dispositions: Array<string>,
}
