import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders} from '@angular/common/http';
import { Router } from '@angular/router';
import { environment } from 'src/enviroments/enviroments';

@Injectable({
  providedIn: 'root'
})
export class LoginService {
  localUrl = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');
  constructor(private http: HttpClient, private router: Router,) { }


  
  public async loginRequest(body: {username: string; password: string;}): Promise<boolean> {
    console.log("login request")
    return this.http.post<any>(this.localUrl + '/login', body).toPromise().then(response => {
      console.log(response)
      if (response){
        this.router.navigate(['board/']);

      } else {
        console.log("no")
        alert("not allowed")
        return false;
      }
      return true;
    }).catch(error => {
      console.error(error);
      return false;
    });
  }

  public canLogin(){
    return false
  }
}
