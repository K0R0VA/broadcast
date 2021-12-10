import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { map, tap } from 'rxjs';

@Component({
  selector: 'app-room',
  templateUrl: './room.component.html',
  styleUrls: ['./room.component.scss'],
})
export class RoomComponent implements OnInit {
  constructor(private router: ActivatedRoute) {}

  ngOnInit(): void {}

  get room() {
    return this.router.data.pipe(
      map(({room}) => room.name)
    );
  }
}
