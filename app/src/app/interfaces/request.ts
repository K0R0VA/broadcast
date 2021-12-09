export type Request = EnterTheRoom | LeaveTheRoom | StartBroadcast | StartWatch;

export interface EnterTheRoom {
  event: 'EnterTheRoom';
  data: string;
}

export interface LeaveTheRoom {
  event: 'EnterTheRoom';
}

export interface StartBroadcast {
  event: 'EnterTheRoom';
  data: string;
}

export interface StartWatch {
  event: 'EnterTheRoom';
  data: {
    local_description: string;
    broadcaster_id: string;
  };
}
