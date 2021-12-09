export type Response = RoomSessions | NewSession | BroadcastDescription | RecipientDescription;

export interface RoomSessions {
  event: 'RoomSessions';
  data: string[];
}

export interface NewSession {
  event: 'NewSession';
  data: string
}

export interface BroadcastDescription {
  event: 'BroadcastDescription';
  data: string;
}

export interface RecipientDescription {
  event: 'RecipientDescription';
  data: {
    description: string;
    broadcaster_id: string;
  };
}