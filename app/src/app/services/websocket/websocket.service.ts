import { Inject, Injectable } from '@angular/core';
import { distinctUntilChanged, interval, map, Observable, Observer, share, Subject, SubscriptionLike, takeWhile } from 'rxjs';
import {WebSocketSubject, WebSocketSubjectConfig} from 'rxjs/webSocket/';
import { WebsocketConfig } from 'src/app/interfaces/websocket-config';
import { config } from './websocket.config';

@Injectable({
  providedIn: 'root'
})
export class WebsocketService<ResponseMessage, RequestMessage> {
  protected config!: WebSocketSubjectConfig<any>;
  protected websocket$!: WebSocketSubject<any>;
  private websocketSub: SubscriptionLike;
  private statusSub: SubscriptionLike;

  private reconnection$!: Observable<number>;

  protected connection$!: Observer<boolean>;
  private wsMessages$: Subject<ResponseMessage>;

  private reconnectInterval: number;
  private readonly reconnectAttempts: number;
  private isConnected!: boolean;

  public status: Observable<boolean>;

  public constructor(@Inject(config) protected wsConfig: WebsocketConfig) {
    this.wsMessages$ = new Subject<ResponseMessage>();

    this.reconnectInterval = wsConfig.reconnectInterval || 5000; // pause between connections
    this.reconnectAttempts = wsConfig.reconnectAttempts || 10; // number of connection attempts

    this.config = {
      url: `${wsConfig.url}`,
      closeObserver: {
        next: () => {
          this.websocket$.unsubscribe();
          this.connection$.next(false);
        },
      },
      openObserver: {
        next: () => {
          console.log("WebSocket connected!");
          this.connection$.next(true);
        },
      },
    };

    this.status = new Observable<boolean>((observer) => {
      this.connection$ = observer;
    }).pipe(share(), distinctUntilChanged());
    this.statusSub = this.status
      .subscribe((isConnected) => {
        this.isConnected = isConnected;
        if (!this.reconnection$ && typeof (isConnected) === 'boolean' && !isConnected) {
          this.reconnect();
        }
      });
    this.websocketSub = this.wsMessages$.subscribe(
      null, (error: ErrorEvent) => console.error('WebSocket error!', error)
    );
    this.connect();
  }

  protected connect(): void {
    this.websocket$ = new WebSocketSubject(this.config);

    this.websocket$.subscribe(
      (event: any) => {
        this.wsMessages$.next(event);
      },
      (error: Event) => {
        if (!this.websocket$) {
          this.reconnect();
        }
      });
  }

  private reconnect(): void {
    this.reconnection$ = interval(this.reconnectInterval)
      .pipe(takeWhile((_, index) => index < this.reconnectAttempts && !this.websocket$));

    this.reconnection$.subscribe(
      () => this.connect(),
      null,
      () => {
        this.reconnection$ = null as unknown as Observable<number>;
        if (!this.websocket$) {
          this.wsMessages$.complete();
          this.connection$.complete();
        }
      });
  }

  public on(): Observable<ResponseMessage> {
    return this.wsMessages$.pipe(
      map((message: ResponseMessage) => {
        console.log(message);
        return message
      })
    );
  }

  public send(data: RequestMessage): void {
    if (this.isConnected) {
      this.websocket$.next(data);
    }
  }

  ngOnDestroy(): void {
    this.websocketSub.unsubscribe();
    this.statusSub.unsubscribe();
  }

}
