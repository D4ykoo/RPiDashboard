import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders} from '@angular/common/http';
import { Router } from '@angular/router';
import { environment } from 'src/enviroments/enviroments';
import { TokenstorageService } from './tokenstorage.service';
import { BoardComponent } from 'src/app/board/board.component';

@Injectable({
  providedIn: 'root'
})
export class LoginService {
  localUrl = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');
  constructor(private http: HttpClient, private router: Router, private storageService: TokenstorageService) { }


  public loginRequest(body: {username: string, password: string}) {
    return this.http.post<any>(this.localUrl + '/login', body).
    subscribe((res: any) => {
      // TODO: Handle res data from server
      if (!res.res){alert("can not login"); return;}
      this.storageService.saveToken("auth-token", res.token)

      this.router.navigate(['/board']);
    });
  }

  public canLogin(){
    let token = this.storageService.getToken("auth-token");
    if (token !== null){
      return true
    }
    return false;
  }
}
