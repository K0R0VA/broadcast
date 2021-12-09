import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RoomComponent } from './room.component';
import { RoomRoutingModule } from './room-routing.module';

@NgModule({
  imports: [
    CommonModule,
    RoomRoutingModule
  ],
  declarations: [RoomComponent]
})
export class RoomModule { }