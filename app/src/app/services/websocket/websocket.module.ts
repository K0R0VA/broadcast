import {ModuleWithProviders, NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import { WebsocketService } from './websocket.service';
import { WebsocketConfig } from 'src/app/interfaces/websocket-config';
import { config } from './websocket.config';


@NgModule({
  imports: [
    CommonModule
  ],
  providers: [
    WebsocketService
  ]
})
export class WebSocketModule {
  public static config(wsConfig: WebsocketConfig): ModuleWithProviders<unknown> {
    return {
      ngModule: WebSocketModule,
      providers: [{provide: config, useValue: wsConfig }]
    };
  }
}
