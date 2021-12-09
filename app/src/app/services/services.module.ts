import { NgModule } from '@angular/core';
import { HttpClientModule } from '@angular/common/http';
import { HttpService } from './http/http-service.service';
import { RouterModule } from '@angular/router';
import { HomeService } from './home/home.service';
import { WebSocketModule } from './websocket/websocket.module';

@NgModule({
  imports: [
      HttpClientModule,
      RouterModule,
      // @ts-ignore
      WebSocketModule.config({reconnectAttempts: 1, reconnectInterval: 500, url: '192.168.0.26:8081/start'})
  ],
  providers: [HttpService, HomeService, WebSocketModule],
})
export class ServicesModule { }
