import { register, init, getLocaleFromNavigator, getLocaleFromQueryString } from 'svelte-i18n';


export function initi18n() {
  register('en', () => import('./lang/en.json'));
  register('es', () => import('./lang/es.json'));
  register('gl', () => import('./lang/gl.json'));

  init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromQueryString("lang") ?? getLocaleFromNavigator(),
  });
}