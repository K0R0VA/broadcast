import { Injectable } from '@angular/core';
import { ActivatedRouteSnapshot, Resolve } from '@angular/router';
import { Observable } from 'rxjs';
import { map, take } from 'rxjs/operators';
import { Room } from '../interfaces/room';
import { HttpService } from '../services/http/http-service.service';

@Injectable({ providedIn: 'root' })
export class RoomResolver implements Resolve<Room> {
  constructor(private service: HttpService) {}

  resolve(route: ActivatedRouteSnapshot): Observable<Room> {
    const roomId: string = String(route.paramMap.get('roomId'));
    return this.service.Get(`room/${roomId}/name`).pipe(
      map((name) => ({
        name: name as string,
        id: roomId,
      })),
      take(1)
    );
  }
}
