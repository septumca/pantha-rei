import type { EventData, ReferenceData, UserData } from 'src/types/prtypes.type';
import { writable } from 'svelte/store';

const eventStore = writable<{ events: Array<EventData> }>({ events: [] });
const userStore = writable<{ users: Array<UserData> }>({ users: [] });
const refDataStore = writable<ReferenceData>({ dispositions: [] });
const loggedInUserStore = writable<UserData | null>(null);

export { eventStore, userStore, refDataStore, loggedInUserStore }