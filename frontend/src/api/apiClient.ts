import { UserCreate, User } from "./models/User";
import { plainToInstance } from "class-transformer";
import { Token } from "./models/Token";

export class ApiClient{
    token: string

    constructor(token: string){
        this.token = token
    }

    static async createUser(userCreate: UserCreate): Promise<User>{
        let headers = new Headers()
        headers.append('content-type', 'application/json')
        var response = await fetch("api/users/", {
            method: "POST",
            headers: headers,
            body: JSON.stringify(userCreate)
        })
        return plainToInstance(User, await response.json())
    }

    static async logIn(username: string, password: string): Promise<Token>{
        let postfields = new URLSearchParams()
        postfields.append("username", username)
        postfields.append("password", password)

        var response = await fetch("api/token", {
            method: "POST",
            body: postfields//`grant_type=&username=${username}&password=${password}&scope=&client_id=&client_secret=`
        })
        return plainToInstance(Token, await response.json())
    }

    async getCurrentUser(): Promise<User>{
        var headers = new Headers()
        headers.append("Authorization", `Bearer ${this.token}`)      
        var response = await fetch("api/users/me",{
            headers: headers
        })
        return plainToInstance(User, await response.json())
    }
}