<script lang="ts">
    import { onMount } from "svelte";
    import Cards from "./Cards.svelte";
    import { commands, type Deck } from "$lib/bindings";

    let decks: Deck[] = [];
    let previous: () => void;
    let advance: () => void;

    onMount(async () => {
        let decksResult = await commands.readDecksFromAnkiFile(
            "C:\\Users\\Lucas\\Downloads\\LearnDutchorg_-_1000_Most_Common_Words_in_Dutch.apkg",
        );

        if (decksResult.status === "ok") {
            decks = decksResult.data;
        } else {
            console.error(decksResult.error);
        }

        document.onkeyup = (e: KeyboardEvent) => {
            if (
                e.key === "ArrowRight" ||
                e.key === "ArrowDown" ||
                e.key === " " ||
                e.key === "Enter" ||
                e.key.toUpperCase() === "D" ||
                e.key.toUpperCase() === "S"
            ) {
                advance();
            } else if (
                e.key === "ArrowLeft" ||
                e.key === "ArrowUp" ||
                e.key === "Backspace" ||
                e.key.toUpperCase() === "W" ||
                e.key.toUpperCase() === "A"
            ) {
                previous();
            }
        };
    });
</script>

<div class="content-wrapper">
    <div class="content">
        {#each decks as deck}
            <Cards cards={deck.cards} bind:previous bind:advance />
        {/each}
    </div>
</div>

<style lang="scss">
    .content-wrapper {
        flex-grow: 1;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .content {
        display: flex;
        align-items: center;
        justify-content: center;
        max-width: 800px;
        max-height: 400px;
        width: 100%;
        height: 100%;
    }
</style>
