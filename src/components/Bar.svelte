<script lang="ts">
  import { commands, type AnswerQuality, type Card } from "$gen/bindings";
  import { onMount } from "svelte";
  import { card } from "../state";
  import RelativeTime from "./RelativeTime.svelte";

  export let state: "Thinking" | "WasRight" | "WasWrong" = "Thinking";

  let previews: Partial<{ [key in AnswerQuality]: string }> | undefined;

  card.subscribe(async (card) => {
    previews = await commands.previewNextReview(card);
  });
</script>

<div class="bar">
  <div class="items">
    {#if previews}
      <div class="item left"><RelativeTime time={previews["OkEasy"]} /></div>
      <div class="item center">
        <RelativeTime time={previews["OkHesitated"]} />
      </div>
      <div class="item right">
        <RelativeTime time={previews["OkDifficult"]} />
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
  $big-border-radius: 40px;
  $small-border-radius: 10px;

  .bar {
    background-color: black;
    padding: 15px;
    border-radius: 40px;
    z-index: 100;
  }

  .bar,
  .items {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .items {
    gap: 3px;
  }

  .item {
    background-color: #1a1a1a;
    color: white;
    padding: 10px 22px;
    border-radius: $small-border-radius;

    &.left {
      border-top-left-radius: $big-border-radius;
      border-bottom-left-radius: $big-border-radius;
    }

    &.right {
      border-top-right-radius: $big-border-radius;
      border-bottom-right-radius: $big-border-radius;
    }
  }
</style>
