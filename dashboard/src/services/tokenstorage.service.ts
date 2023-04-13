import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class TokenstorageService {

  constructor() { }
  /**
   * Clears whole session storage on call
   */
  public clearStorage(): void {
    window.sessionStorage.clear();
  }

  /**
   * Removes one item of the session storage
   * 
   * @param tk: TokenKey that will be removed
   */
  public removeToken(tk: string){
    window.sessionStorage.removeItem(tk);
  }

  /**
   * Stores one item in the session storage
   * 
   * @param tk Token Key e.g. auth-token-invar
   * @param token Token value stored 
   */
  public saveToken(tk: string, token: string): void {
    window.sessionStorage.removeItem(tk);
    window.sessionStorage.setItem(tk, token);
  }

  /**
   * Returns the value of the requested token key.
   * 
   * @param tk Token Key
   * @returns The value of the token key as string or null when empty
   */
  public getToken(tk: string): string | null {
    return window.sessionStorage.getItem(tk);
  }


  /**
   * Checks if user is logged in depending on the auth token.
   * 
   * @returns true or false
   */
  public isLoggedIn(): boolean {
    const user = window.sessionStorage.getItem("auth-token");
    if (user) {
      return JSON.parse(user);
    }
    return false;
  }
}
