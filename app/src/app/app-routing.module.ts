import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { RoomResolver } from './resolvers/room.resolver';

const routes: Routes = [
  {
    path: '',
    loadChildren: () => import('./pages/home/home.module').then(m => m.HomeModule),
  },
  {
    path: 'room/:roomId',
    loadChildren: () => import('./pages/room/room.module').then(m => m.RoomModule),
    resolve: {
      room: RoomResolver
    }
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
