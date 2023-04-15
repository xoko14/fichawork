import { User } from './api/models/User';
import { writable, derived } from 'svelte/store';
import { Section } from './lib/Sections';

export const currentUser = writable(new User())

export const token = writable(localStorage.getItem("token")?? "")
token.subscribe(value => {
    localStorage.setItem("token", value)
})

export const activeSection = writable(Section[localStorage.getItem("section")]?? Section.LOGIN)
activeSection.subscribe(value =>
    localStorage.setItem("section", Section[value])
)

export const message = writable("")