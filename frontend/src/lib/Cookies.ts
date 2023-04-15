export class Cookies{
    static setCookie(c_name: string, c_value: string, exDate: Date | null) {

        document.cookie=encodeURIComponent(c_name) 
          + "=" + encodeURIComponent(c_value)
          + (exDate ? "" : "; expires="+exDate.toUTCString());
          ;
     }
}