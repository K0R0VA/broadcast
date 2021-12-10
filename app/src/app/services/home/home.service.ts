import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { tap } from 'rxjs';
import { HttpService } from '../http/http-service.service';

@Injectable({
  providedIn: 'root'
})
export class HomeService {

  constructor(private httpService: HttpService, private router: Router) { }

  createRoom(roomName: string) {
    return this.httpService.Post('create-room', roomName).pipe(
      tap((id) => {
        this.router.navigate(['room', id])
      })
    )
  }
  
}
