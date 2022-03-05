import type { EventData, Fullfillment, ReferenceData, UserData } from 'src/types/prtypes.type';
import { writable } from 'svelte/store';

const eventStore = writable<{ events: Array<EventData> }>({ events: [] });
const userStore = writable<{ users: Array<UserData> }>({ users: [] });
const refDataStore = writable<ReferenceData>({ dispositions: [] });

export const setFullfillmentToEvent = (eventId: string, requirement_name: string, fullfillment: Fullfillment | null) => {
  eventStore.update(es => ({ ...es, events: es.events.map(e => {
    if (e._id !== eventId) {
      return e;
    }
    return {
      ...e,
      requirements: e.requirements.map(er => {
        if(er.name !== requirement_name) {
          return er;
        }
        return { ...er, fullfilled_by: fullfillment };
      })
    };
  })}));
}

export const addEvent = (event: EventData) => {
  eventStore.update(d => ({ ...d, events: [...d.events, event] }));
}

export const removeEvent = (eventId: string) => {
  eventStore.update(d => ({ ...d, events: d.events.filter(e => e._id !== eventId)}));
}

export const addUserToEventStore = (eventId: string, user: UserData) => {
  eventStore.update(d => ({ ...d, events: d.events.map(e => e._id === eventId ? {
    ...e,
    participants: [...e.participants, user],
  } : e)}))
}

export const removeUserFromEventStore = (eventId: string, userId: string) => {
  console.info(eventStore);
  eventStore.update(d => ({ ...d, events: d.events.map(e => e._id === eventId ? {
    ...e,
    participants: e.participants.filter(u => u._id !== userId),
    requirements: e.requirements.map(r => {
      return r.fullfilled_by !== null && r.fullfilled_by._id === userId ? { ...r, fullfilled_by: null } : r
    })
  } : e)}));
}

export const addUser = (user: UserData) => {
  userStore.update(d => ({ ...d, users: [...d.users, user] }));
}

export const removeUser = (userId: string) => {
  userStore.update(d => ({ ...d, users: d.users.filter(e => e._id !== userId)}));
}

export { eventStore, userStore, refDataStore };
