import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Component } from '@angular/core';
import { Route, Router } from '@angular/router';
import { environment } from 'src/enviroments/enviroments';

@Component({
  selector: 'app-board',
  templateUrl: './board.component.html',
  styleUrls: ['./board.component.css']
})
export class BoardComponent {
  localUrl = environment.apiUrl;
  headers = new HttpHeaders().set('Content-Type', 'application/json');

  constructor(private http: HttpClient, private router: Router,) { }
    public createUser(): Promise<boolean> {
        return this.http.post<any>(this.localUrl + '/create', {}).toPromise().then(response => {
          console.log(response)
          if (response){
            
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
    }

