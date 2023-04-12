import { Component } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import sha256 from 'fast-sha256';
import { TokenstorageService } from 'src/services/tokenstorage.service';
import { UsermanagementService } from 'src/services/usermanagement.service';

@Component({
  selector: 'app-usermanagement',
  templateUrl: './usermanagement.component.html',
  styleUrls: ['./usermanagement.component.css']
})
export class UsermanagementComponent {
  createForm !: FormGroup;
  deleteForm !: FormGroup;
  submitted!: boolean;

  constructor(
    public formBuilder: FormBuilder,
    private userService: UsermanagementService,
    private storageService: TokenstorageService,
    ) { }

  ngOnInit(): void {
      this.createForm = this.formBuilder.group({
        username: ['', Validators.required],
        password: ['', Validators.required]
      });
      
      this.deleteForm = this.formBuilder.group({
        username: ['', Validators.required],
      });
  }
  public onCreate(){
    // this.submitted = true;    

    if(this.createForm.invalid){
      return;
    }
    let password_un = this.createForm.value.password;
    let hashed = Array.from(new Uint8Array(sha256(password_un))).map((bytes) => bytes.toString(16).padStart(2, '0')).join('');

    const body = {
      username: this.createForm.value.username,
      password: hashed,
      token: this.storageService.getToken("auth-token") || ""
    };

    this.userService.createUserRequest(body);
  }

  public onDelete(){
    if(this.deleteForm.invalid){
      return;
    }

    const body = {
      username: this.deleteForm.value.username,
      password: "",
      token: this.storageService.getToken("auth-token") || ""
    };

    this.userService.deleteUserRequest(body);
  }
}
