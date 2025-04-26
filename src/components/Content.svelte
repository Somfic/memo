<script lang="ts">
    import { onMount } from "svelte";
    import Cards from "./Cards.svelte";
    import { commands, type Deck } from "$gen/bindings";

    let decks: Deck[] = [];
    let previous: () => void;
    let advance: () => void;
    let flip: () => void;

    let isFlipped = false;

    onMount(async () => {
        let decksResult = await commands.readDecksFromAnkiFile(
            "/Users/lucas/Downloads/English_Italian_-_Assimil_words_Idiom_irregular_verbs.apkg",
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
                if (!isFlipped) {
                    isFlipped = true;
                    flip();
                } else {
                    isFlipped = false;
                    advance();
                }
            } else if (
                e.key === "ArrowLeft" ||
                e.key === "ArrowUp" ||
                e.key === "Backspace" ||
                e.key.toUpperCase() === "W" ||
                e.key.toUpperCase() === "A"
            ) {
                if (isFlipped) {
                    isFlipped = false;
                    flip();
                } else {
                    previous();
                }
            }
        };
    });
</script>

<div class="content-wrapper">
    <div class="content">
        {#each decks as deck}
            <Cards cards={deck.cards} bind:previous bind:advance bind:flip />
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
