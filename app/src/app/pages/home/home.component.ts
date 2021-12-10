import { Component, OnInit } from '@angular/core';
import { HomeService } from 'src/app/services/home/home.service';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
  name: string = '';
  constructor(private homeService: HomeService) { }

  ngOnInit(): void {
  }

  createRoom() {
    console.log(this.name);
    this.homeService.createRoom(this.name).subscribe();
  }
}
