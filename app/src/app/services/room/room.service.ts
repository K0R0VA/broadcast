import { Injectable } from '@angular/core';
import { filter, Observer } from 'rxjs';
import { EnterTheRoom, LeaveTheRoom, Request, StartBroadcast, StartWatch } from 'src/app/interfaces/request';
import { BroadcastDescription, NewSession, RecipientDescription, Response, RoomSessions } from 'src/app/interfaces/response';
import { WebsocketService } from '../websocket/websocket.service';

@Injectable({
  providedIn: 'root'
})
export class RoomService {

  constructor(private websocket: WebsocketService<Response, Request>) { }

  enterRoom(request: EnterTheRoom) {
    this.websocket.send(request);
  }
  leaveRoom(request: LeaveTheRoom) {
    this.websocket.send(request);
  }
  startBroadcasting(request: StartBroadcast) {
    this.websocket.send(request);
  }
  startWatching(request: StartWatch) {
    this.websocket.send(request);
  }

  get onRoomSessions(): Observer<RoomSessions> {
    return this.websocket.on().pipe(
      filter(response => response.event === 'RoomSessions')
    ) as unknown as Observer<RoomSessions>
  }

  get onNewSession(): Observer<NewSession> {
    return this.websocket.on().pipe(
      filter(response => response.event === 'NewSession')
    ) as unknown as Observer<NewSession>
  }

  get onBroadcastDescription(): Observer<BroadcastDescription> {
    return this.websocket.on().pipe(
      filter(response => response.event === 'BroadcastDescription')
    ) as unknown as Observer<BroadcastDescription>
  }

  get onRecipientDescription(): Observer<RecipientDescription> {
    return this.websocket.on().pipe(
      filter(response => response.event === 'RecipientDescription')
    ) as unknown as Observer<RecipientDescription>
  }
}
