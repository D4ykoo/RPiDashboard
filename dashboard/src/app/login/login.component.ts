import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { LoginService } from 'src/services/login.service';
import sha256, { Hash, HMAC } from "fast-sha256";

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent {
  loginForm !: FormGroup;
  submitted!: boolean;

  constructor(
    public formBuilder: FormBuilder,
    private loginService: LoginService,
    ) { }

  ngOnInit(): void {
      this.loginForm = this.formBuilder.group({
        username: ['', Validators.required],
        password: ['', Validators.required]
      });
  }

  public async onLoginSubmit(){
    // this.submitted = true;    

    if(this.loginForm.invalid){
      return;
    }
    let password_un = this.loginForm.value.password;
    let hashed = Array.from(new Uint8Array(sha256(password_un))).map((bytes) => bytes.toString(16).padStart(2, '0')).join('');

    const body = {
      username: this.loginForm.value.username,
      password: hashed,
    };

    this.loginService.loginRequest(body);
  }
}
