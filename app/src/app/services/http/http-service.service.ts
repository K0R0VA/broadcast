import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
  providedIn: 'root'
})
export class HttpService {

  private readonly base_path: string = 'https://192.168.0.26:8081';

  constructor(private http: HttpClient) { }

  Get<T>(request: string): Observable<T> {
    return this.http.get(`${this.base_path}/${request}`) as Observable<T>;
  }
  Post<T, B>(request: string, body: B): Observable<T> {
    return this.http.post(`${this.base_path}/${request}`, body) as Observable<T>;
  }
}
