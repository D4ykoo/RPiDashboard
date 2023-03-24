import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { LoginService } from 'src/services/login.service';

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
  public onLoginSubmit(){
    // this.submitted = true;    

    if(this.loginForm.invalid){
      return;
    }

    const body = {
      username: this.loginForm.value.username,
      password: this.loginForm.value.password
    };

    console.log(body)
    this.loginService.loginRequest(body);
  }
}
