export type UserData = {
  _id: string,
  name: string,
  dispositions: Array<string>
}

export type Fullfillment = {
  _id: string,
  name: string
}

export type Requirement = {
  name: string,
  fullfilled_by: Fullfillment | null,
}

export type EventData = {
  _id: string,
  creator: UserData,
  name: string,
  description: string,
  participants: Array<UserData>,
  requirements: Array<Requirement>,
}

export type ReferenceData = {
  dispositions: Array<string>,
}
