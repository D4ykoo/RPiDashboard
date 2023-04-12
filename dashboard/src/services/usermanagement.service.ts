import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { environment } from 'src/enviroments/enviroments';
import { TokenstorageService } from './tokenstorage.service';

@Injectable({
  providedIn: 'root'
})
export class UsermanagementService {
  localUrl = environment.apiUrl;

  constructor(
    private http: HttpClient, private router: Router, private storageService: TokenstorageService
  ) { }

  public createUserRequest(body: {username: string, password: string, token: string}) {
    return this.http.post<any>(this.localUrl + '/create', body).
    subscribe((res: any) => {
      if (!res.res) {
        alert("Can not create user!")
      } else {
        alert("User created!")
      }
    });
  }

  public deleteUserRequest(body: {username: string, password: string, token: string}) {
    return this.http.post<any>(this.localUrl + '/delete', body).
    subscribe((res: any) => {
      console.log(res);
      if (res.res) {
        alert("User deleted!")
      } 
    });
  }

}
