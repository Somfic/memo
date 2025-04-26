<script lang="ts">
    import { Flame } from "@lucide/svelte";
    import type { Card as CardType } from "$gen/bindings";

    export let card: CardType;
    export let streak: number | undefined = undefined;
    export let isInBackground: boolean;
    export let isFlipped: boolean = false;
</script>

<div
    class="card"
    class:is-in-background={isInBackground}
    class:is-flipped={isFlipped}
>
    <div class="card-content prompt" class:is-in-background={isInBackground}>
        {@html card.prompt.value}
    </div>
    <div class="card-content answer" class:is-in-background={isInBackground}>
        {@html card.answer.value}
    </div>
</div>

<style lang="scss">
    .card {
        background-color: #ffffff;
        border-radius: 30px;

        font-size: 30px;
        box-shadow: 0 0 40px rgba(0, 0, 0, 0.15);
        user-select: none;

        transition: all 0.3s ease-in-out;

        width: 600px;
        height: 300px;

        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        perspective: 100000px;
        transform-style: preserve-3d;

        :global(.code) {
            font-family: "Courier New", Courier, monospace;
        }

        :global(div) {
            display: inline-block;
        }

        .card-content {
            position: absolute;
            transition: opacity 0.3s ease-in-out;
            display: flex;
            flex-grow: 1;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 50px;
            backface-visibility: hidden;

            &.is-in-background {
                opacity: 0;
            }
        }

        .prompt {
            font-size: 30px;
            font-weight: bold;
            color: #333;
            text-align: center;
        }

        .answer {
            color: #333;
            text-align: center;
        }

        .streak {
            position: absolute;
            display: flex;
            align-items: center;
            justify-content: center;
            top: 10px;
            right: 10px;
            color: orange;
            padding: 5px 10px;
            border-radius: 5px;
            font-size: 16px;
        }

        &.is-flipped {
            transform: rotateX(180deg);
        }

        .card-content.answer {
            transform: rotateX(180deg);
        }
    }
</style>
