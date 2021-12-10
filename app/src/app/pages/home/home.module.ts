import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { HomeComponent } from './home.component';
import { HomeRoutingModule } from './home-routing.module';
import { NbButtonModule, NbCardModule, NbInputModule } from '@nebular/theme';
import { FormsModule } from '@angular/forms';

@NgModule({
  imports: [
    CommonModule,
    HomeRoutingModule,
    NbCardModule,
    NbInputModule,
    NbButtonModule,
    FormsModule,
  ],
  declarations: [HomeComponent]
})
export class HomeModule { }