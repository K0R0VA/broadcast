import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RoomComponent } from './room.component';
import { RoomRoutingModule } from './room-routing.module';
import { NbCardModule } from '@nebular/theme';
import { RouterModule } from '@angular/router';

@NgModule({
  imports: [
    CommonModule,
    RoomRoutingModule,
    NbCardModule,
    RouterModule
  ],
  declarations: [RoomComponent]
})
export class RoomModule { }