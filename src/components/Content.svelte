<script lang="ts">
    import { onMount } from "svelte";
    import Cards from "./Cards.svelte";

    let previous: () => void;
    let advance: () => void;

    onMount(() => {
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
        <Cards
            words={[
                "A",
                "B",
                "C",
                "D",
                "E",
                "F",
                "G",
                "H",
                "I",
                "J",
                "K",
                "L",
                "M",
                "N",
                "O",
                "P",
                "Q",
                "R",
                "S",
                "T",
                "U",
                "V",
                "W",
                "X",
                "Y",
                "Z",
            ]}
            bind:previous
            bind:advance
        />
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
