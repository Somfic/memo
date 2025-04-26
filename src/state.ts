import { type Card } from "$gen/bindings";
import { writable } from "svelte/store";

export let card = writable<Card>();
